from pathlib import Path
from jplephem.spk import SPK

G = 6.67408e-11

DIR_PATH = Path(__file__).parent.parent.resolve()
YAML_PATH = DIR_PATH / 'config'
EPHEMERIS_PATH = DIR_PATH / 'config' / 'de440.bsp'

def eph_data():
    return SPK.open(EPHEMERIS_PATH)