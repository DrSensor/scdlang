#!/usr/bin/env python
from tomlkit.toml_file import TOMLFile
from glob import glob
from os import path
from sys import argv, stdin
from pampy import match
from functools import reduce
import operator as op
import re

re_version = r"\d+\.\d+\.\d+-?"


def increment(version, major=None, minor=None, patch=None):
    version = v = [int(ver) for ver in version.split(".")]
    if isinstance(major, int):
        version = [v[0] + major, 0, 0]
    if isinstance(minor, int):
        version = [v[0], v[1] + minor, 0]
    if isinstance(patch, int):
        version = [v[0], v[1], v[2] + patch]
    return ".".join([str(ver) for ver in version])


# fmt: off
def change_version(version):
    return match(
        argv[1],
        "major", increment(version, major=1),
        "minor", increment(version, minor=1),
        "patch", increment(version, patch=1),
        re.compile(re_version), lambda target: target.strip("-"),
        "major-", increment(version, major=-1),
        "minor-", increment(version, minor=-1),
        "patch-", increment(version, patch=-1),
    )


def docker_release():
    re_sep = r"(?:=|\s+)"
    re_version_label = r"(version%s[\"']?(%s)[\"']?)" % (re_sep, re_version)
    for docker_file in glob("docker/*.Dockerfile"):
        with open(docker_file, "r+") as file:
            dockerfile = file.read()
            (version, v) = re.findall(re_version_label, dockerfile, re.IGNORECASE)[0]
            new_version = re.sub(re_version, change_version(v), version)
            file.seek(0)  # workaround for read & overwrite file
            file.write(dockerfile.replace(version, new_version))
            file.truncate()


def cargo_release(project, internal_dependencies=[None]):
    project_path = path.join(project, "Cargo.toml")
    file = TOMLFile(project_path)
    content = file.read()
    dependencies = content.get('dependencies') or {}
    build_dependencies = content.get('build-dependencies') or {}
    new_version = change_version(content['package']['version'])

    content['package']['version'] = new_version
    for local in internal_dependencies:
        if dependencies.get(local) is not None:
            dependencies[local]['version'] = new_version
        if build_dependencies.get(local) is not None:
            build_dependencies[local]['version'] = new_version

    file.write(content)


def cargo_workspace_release():
    workspace = TOMLFile("Cargo.toml").read()['workspace']
    paths = reduce(op.concat, [glob(p) for p in workspace['members']], [])
    project_names = [TOMLFile(f"{path}/Cargo.toml").read()['package']['name'] for path in paths]
    for project in paths:
        cargo_release(project, project_names)


if not stdin.isatty():
    print(change_version(stdin.read()))
else:
    cargo_workspace_release()
    docker_release()
