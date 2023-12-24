from pathlib import Path
from jplephem.spk import SPK
from functools import lru_cache

G = 6.67408e-11

DE440_PATH = Path("../_main~download_de440~de440/file/downloaded")

@lru_cache
def eph_data():
    return SPK.open(DE440_PATH)


def test_open_eph_data():
    eph_data()


if __name__ == "__main__":
    test_open_eph_data()
