#!/usr/bin/env python
from tomlkit.toml_file import TOMLFile
from glob import glob
from os import path
from sys import argv, stdin
from pampy import match
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


def cargo_release(project):
    project_path = path.join(project, "Cargo.toml")
    file = TOMLFile(project_path)
    content = file.read()
    version = content['package']['version']
    content['package']['version'] = change_version(version)
    file.write(content)


def cargo_workspace_release():
    workspace = TOMLFile("Cargo.toml").read()['workspace']
    for project in workspace['members']:
        if "*" in project:
            for project_abspath in glob(project):
                cargo_release(project_abspath)
        else:
            cargo_release(project)


if not stdin.isatty():
    print(change_version(stdin.read()))
else:
    cargo_workspace_release()
    docker_release()
