#! /usr/bin/env python
# -*- coding: utf-8 -*-
# vim:fenc=utf-8
#
# Copyright © 2022 fx-kirin <fx.kirin@gmail.com>
#
# Distributed under terms of the MIT license.

"""

"""

import sqlalchemy.types as types
from . import Decimal

class Numeric(types.Numeric):
    def bind_processor(self, dialect):
        breakpoint()
        def process(value):
            return Decimal(value)
        return process

    def result_processor(self, dialect, coltype):
        breakpoint()
        def process(value):
            return value
        return process
