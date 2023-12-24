from pathlib import Path
from jplephem.spk import SPK
from functools import lru_cache

G = 6.67408e-11

DIR_PATH = Path(__file__).parent.resolve()
# YAML_PATH = DIR_PATH / "data" / "sol_system.yaml"
EPHEMERIS_PATH = Path("de440")


@lru_cache
def eph_data():
    return SPK.open(EPHEMERIS_PATH)


def test_open_eph_data():
    eph_data()


if __name__ == "__main__":
    test_open_eph_data()
