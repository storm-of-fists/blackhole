from pathlib import Path

G = 6.67408e-11

DIR_PATH = Path(__file__).parent.parent.resolve()
YAML_PATH = DIR_PATH / 'instantiation_data'
EPHEMERIS_PATH = DIR_PATH / 'instantiation_data' / 'de440.bsp'