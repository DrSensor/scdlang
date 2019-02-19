#!/bin/env python3
import re,sys
from itertools import zip_longest
from subprocess import run

assert len(sys.argv) > 1

# grouper('ABCDEFG', 3, 'x') --> ABC DEF Gxx"
def grouper(iterable, n, fillvalue=None):
  "Collect data into fixed-length chunks or blocks"
  args = [iter(iterable)] * n
  return zip_longest(*args, fillvalue=fillvalue)

def group_command(flatten_commands, commands):
  "Group string by it's command name"
  separator = f"({'|'.join(list_command)})"
  separated_cmd_params = [s.strip() for s in re.split(separator, flatten_commands)[1:]]
  return [' '.join(s) for s in grouper(separated_cmd_params, 2)]

cargo_list = run(['cargo', '--list'], capture_output=True)
stdout = cargo_list.stdout.decode('utf-8')

command_help = stdout.split('\n')[1:-1]             # remove endline and title string
list_command =[s.split(maxsplit=1)[0] for s in command_help]  # remove help description

grouped = group_command(' '.join(sys.argv[1:]), list_command)

prefix_command = lambda f: '"{prefix} {}"'.format(f'" "{f} '.join(grouped), prefix=f)
print(prefix_command('cargo'))
