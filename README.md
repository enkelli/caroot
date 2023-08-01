# Caroot <img src="web/frontend/src/logo.svg" width="40" height="40">

Uniform multi-state one-dimensional cellular automata (CA). Caroot attempts to solve the problem of square root calculations using an evolutionary algorithm.

This project consists of these parts:

* [caroot](#caroot) - implementation of an evolutionary algorithm.
* [cartist](#cartist) - GUI/CLI calculations with the found CA rule sets.

## Caroot

Rust program that attempts to find transition functions for square root computation with simple evolutionary algorithm.

### Usage

```
make run
```

Evolutionary algorithm parameters can be set in [`rust/config.conf`](rust/config.conf).

#### Output

During development it prints current generation and fitness when better rule set is found (to `stderr`).

When function with fitness 0 or maximum number of generations is reached, the best rule-set is printed.

### Rule format

Currently, only one format is supported.

#### CSV

```
s1,s2,...,sN,t1
s1,s2,...,sN,t2
```

Each line contains one rule. Each rule describes N states `s` that decide about new state `t` for a cell.

## Cartist

GUI and CLI tools to test/represent the generated transition functions (rule-set).

GUI requires [tkinter](https://docs.python.org/3/library/tkinter.html) installed.

### Run

```
# Runs some default automaton
./cartist.py

# Runs automaton with the given rule set and computes square root of 36.
./cartist.py -r rulesets/sq-4-11 36

# CLI version of the above.
./cartist.py -r rulesets/sq-4-11 16

---................---
---1..............----
---1O............-----
---.O...........------
---.1..........-------
---11O........--------
---1OO.......---------
---.OO......----------
---..O.....-----------
---111....------------
---111O..-------------
---11OO.--------------
---1OOO.--------------
---.OOO.--------------
---..OO.--------------
---11.O.--------------
---1111.--------------
---1111---------------
---1111---------------
√16: 4

# Prints just square root of the given number.
./cartist.py -r rulesets/sq-4-11 -s 36
6
```

Use `-g` flag to run GUI version of the above commands. It requires [`tkinter`](https://www.pythonguis.com/installation/install-tkinter-linux/).

### Examples

Square root 9, 16 and 25

![caroot9](https://user-images.githubusercontent.com/14038418/115896409-b0caec00-a45b-11eb-873a-cc85c288f33a.png)
![caroot16](https://user-images.githubusercontent.com/14038418/115896004-4ade6480-a45b-11eb-976c-22316485da6d.png)
![caroot25](https://user-images.githubusercontent.com/14038418/115896415-b1fc1900-a45b-11eb-8719-ab967096fdb7.png)


## Credits

This project is extension of my school project at [FIT BUT](https://www.fit.vut.cz/.en) - [Bio-Inspired Computers](https://www.fit.vut.cz/study/course/BIN/.en), inspired by [Evolution of Generic Square Calculations in Cellular Automata](https://www.scitepress.org/PublicationsDetail.aspx?ID=fUDdabZdceo=&t=1) by [Ing. Michal Bidlo, Ph.D.](https://www.fit.vut.cz/person/bidlom/.en).
