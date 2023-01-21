import csv
import yaml
# from constants import PATH
import constants

class DataHandler:

    output_path = constants.DIR_PATH / "output"
    output_type = '.csv'
    output_data = {}

    def write_data(self, obj):
        file_name = obj.name + self.output_type
        file = self.output_path / file_name
        with file.open(mode='w') as f:
            f_writer = csv.writer(f, delimiter=',')
            for pos in self.output_data[obj.name]['pos']:
                f_writer.writerow(pos)

    def store_data(self, obj):
        if obj.name not in self.output_data:
            self.output_data[obj.name] = obj_data = {}
            obj_data['pos'] = [obj.state.pos_i]
            # obj_data['vel'] = [obj.state.vel_i]
        else:
            obj_data = self.output_data[obj.name]
            obj_data['pos'].append(obj.state.pos_i)
            # obj_data['vel'].append(obj.state.vel_i)


def parse_yaml(yaml_file):
    with yaml_file.open() as f:
        yaml_data = yaml.safe_load(f)
    
    return yaml_data