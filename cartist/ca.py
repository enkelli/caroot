#!/usr/bin/env python3


class SqrtCA:
    """Simple 1D cellular automaton."""

    def __init__(self, ruleset=None, number=0, max_iters=10000):
        self._ruleset = ruleset if ruleset else {}
        self._number = number
        self._rule_len = self._ruleset.rule_len
        self._boundary_len = self._rule_len
        self._row_len = number + 2 * self._boundary_len

        self._ca = [1 for _ in range(self._row_len)]
        self._ca[: self._boundary_len] = [0] * self._boundary_len
        self._ca[-self._boundary_len :] = [0] * self._boundary_len
        self._ca_len = len(self._ca)
        self._changed = False
        self._max_iters = max_iters

    def develop(self, generations=None):
        """Develops the CA. When `generations` is ``None``, it evolves CA until
        it is stable.
        """
        iters = 0
        while True:
            self.evolve()
            if not self.has_changed():
                return
            iters += 1
            if iters > self._max_iters:
                break

    def evolve(self):
        """Runs one evolutionary step on the automaton."""
        new_ca = [self._ruleset.get(chunk) for chunk in self._iter_ca_for_evolution()]
        self._changed = self._ca != new_ca
        self._ca = new_ca

    @property
    def number(self):
        """Returns the initial number passed to the automaton."""
        return self._number

    def has_changed(self):
        """Was the automaton changed after the last :func:`evolve()` call?"""
        return self._changed

    @property
    def sqrt_value(self):
        """Returns current value of the automaton."""
        return sum(1 for cell in self._ca if cell != 0)

    def _iter_ca_for_evolution(self):
        for i in range(self._ca_len):
            chunk = []
            for j in range(i - self._ruleset.radius, i + self._ruleset.radius + 1):
                cell = self._ca[j] if j >= 0 and j < self._ca_len else 0
                chunk.append(cell)
            yield tuple(chunk)

    def __iter__(self):
        return iter(self._ca)

    def __len__(self):
        return len(self._ca)

    def __eq__(self, other):
        return self._ca == other._ca
