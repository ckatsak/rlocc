# This file is part of rlocc.
#
# Copyright (C) 2020 Christos Katsakioris
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

CARGO = cargo

.PHONY: all build debug lint test clean

all: build

build:
	$(CARGO) build --release

debug:
	$(CARGO) build

lint:
	$(CARGO) clippy

test:
	$(CARGO) test -- --show-output

clean:
	$(CARGO) clean