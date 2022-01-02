#!/usr/bin/env python3
# coding: utf-8

import os
import sys
import json

TEST_FILE_HEADER = 'mod common;\n\n'
TEST_FUNC_TEMPLATE = '''\
#[test]
fn opcode_0x{code}_{mode}_{name}() {{
    let mut _cpu = common::init_cpu();
    assert_eq!(2 + 2, 5);
}}\n'''

OPCODE_TEMPLATE = '(0x{code}, Opcode::new(0x{code}, {size}, {tick}, "{name}", \
    AddressingMode::{mode}, TickModifier::None)),'.replace('    ', '')
_MATCH_TEMPLATE = '{codes} => self.{name}(&opcode.mode)'

GET_PATH = lambda *p: os.path.join(os.path.dirname(sys.argv[0]), '..', *p)


def generate_test_file(instructions):
    print(
        TEST_FILE_HEADER +
        '\n'.join(map(
            lambda ins: TEST_FUNC_TEMPLATE.format(
                code=ins['code'],
                mode=ins['mode'].lower(),
                name=ins['name'].lower()), instructions)).rstrip(),
        file=open(GET_PATH('tests', f'test_opcode_{instructions[0]["name"].lower()}.rs'), 'w'),
        end='')


def generate_code_file(instructions):
    print('\n'.join(
        map(lambda ins: OPCODE_TEMPLATE.format(**ins), instructions)))


def main(argc, argv):
    match argc:
        case 3:
            instructions, opcode = argv[1:]
        case 2:
            instructions, opcode = GET_PATH('scripts', 'instructions.json'), argv[1]
        case _:
            return print(f'{argv[0]} [instructions.json] OPCODE') or 1

    instructions = list(filter(
        lambda op: op['name'] == opcode.upper(),
        json.load(open(instructions, 'r'))))

    generate_test_file(instructions)
    generate_code_file(instructions)

    print('\n' +
        _MATCH_TEMPLATE.format(
            name=opcode.lower(),
            codes=' | '.join(
                map(lambda ins: f'0x{ins["code"]}', instructions))))

    return 0


if __name__ == '__main__':
    exit(main(len(sys.argv), sys.argv))