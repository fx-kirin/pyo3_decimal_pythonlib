#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8

import logging
import os

import kanilog
import pytest
import stdlogging
from add_parent_path import add_parent_path

with add_parent_path():
    from pyo3_decimal import Decimal


def setup_module(module):
    pass


def teardown_module(module):
    pass


def setup_function(function):
    pass


def teardown_function(function):
    pass


def test_func():
    assert Decimal("12.3") == Decimal(123, 1)
    Decimal(123.4)
    Decimal(123, 3)
    assert Decimal(2, 3) == Decimal(2, 3)
    Decimal(2, 3) * Decimal(2, 4)
    Decimal(2, 3) * 2
    Decimal(2, 3) / Decimal(2, 4)
    Decimal(2, 3) / 2
    Decimal(2, 3) + Decimal(2, 4)
    Decimal(2, 3) - Decimal(2, 4)
    - Decimal(2, 3)
    assert Decimal(2, 3) + 2 == Decimal(2002, 3)
    pass


if __name__ == "__main__":
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    kanilog.setup_logger(logfile='/tmp/%s.log' % (os.path.basename(__file__)), level=logging.INFO)
    stdlogging.enable()

    pytest.main([__file__, '-k test_', '-s'])
