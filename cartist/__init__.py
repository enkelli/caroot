"""
    Cellular Automata ARTIST.
"""

from cartist.ca import SqrtCA
from cartist.rules import DEFAULT_RULE_SET


class Artist:
    """Cellular Automata runner."""

    def __init__(self, number, ruleset=DEFAULT_RULE_SET):
        self._ruleset = ruleset
        self._ca = SqrtCA(ruleset, number)

    @property
    def sqrt(self):
        self._ca.develop()
        return self._ca.sqrt_value

    def steps(self):
        yield list(self._ca)
        while True:
            self._ca.evolve()
            yield list(self._ca)
            if not self._ca.has_changed():
                return

    def paint(self):
        self._ca.develop()
        print(self._ca.sqrt_value)
