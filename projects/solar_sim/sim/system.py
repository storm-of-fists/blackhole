import numpy as np
from const import (
    G,
)
from util import parse_yaml
from jplephem.spk import SPK
from astropy.time import Time


class State:

    def __init__(self, pos=np.zeros(3), vel=np.zeros(3)):

        self.pos_i = self.pos_n = pos
        self.vel_i = self.vel_n = vel
        
        self.force_n = np.array([0.0, 0.0, 0.0])
        self.acc_n = np.array([0.0, 0.0, 0.0])
        
        self.vel_k = np.array([0.0, 0.0, 0.0])
        self.acc_k = np.array([0.0, 0.0, 0.0])

    def combine(self, other_state):
        self.pos_n += other_state.pos_n
        self.vel_n += other_state.vel_n


class OrbitalBody:

    def __init__(self, name, mass, state, parent_body=None, radius=0.1):
        self.name = name
        self.mass = mass
        self.state = state
        self.radius = radius
        if parent_body:
            self.set_parent_body(parent_body)

    def set_parent_body(self, parent_body):
        self.parent_body = parent_body
        self.state.combine(parent_body.state)

    def force_rel(self, body_2):
        # returns a force vec from body_1 to body_2
        return self.mass*self.acc_rel(body_2)
    
    def acc_rel(self, body_2):
        # returns a force vec from body_1 to body_2
        body_1 = self
        
        diff_vec = body_2.state.pos_n - body_1.state.pos_n
        r_mag = (diff_vec[0]**2 + diff_vec[1]**2 + diff_vec[2]**2)**0.5
        dir_vec = diff_vec/r_mag
        
        if r_mag > self.radius:
            a_mag = G*body_2.mass/r_mag**2
        else:
            a_mag = 0.0

        return a_mag*dir_vec
        

class OrbitalSystem():

    def __init__(self, name, bodies):
        self.name = name
        self.bodies = bodies

    def apply_accs(self):
        for body in self.bodies.values():
            self.apply_acc(body)

    def apply_acc(self, body):
        body.state.acc_n = np.array([0.0, 0.0, 0.0])
        for body_x in [b for b in self.bodies.values() if b != body]:
            body.state.acc_n += body.acc_rel(body_x)

    @classmethod
    def create_from_yaml(cls, yaml_path):
        yaml_dict = parse_yaml(yaml_path)
        name = yaml_dict['name']
        bodies = {}
        for body in yaml_dict['bodies']:
            properties = yaml_dict['bodies'][body]
            
            mass = properties['mass']
            pos = np.array(properties.get('pos', [0.0, 0.0, 0.0]))
            vel = np.array(properties.get('vel', [0.0, 0.0, 0.0]))
            parent = bodies.get(properties.get('parent'), None)
            radius = properties.get('radius', 0.1)
    
            state = State(pos, vel)
            body_obj = OrbitalBody(body, mass, state, parent_body=parent, radius=radius)
            bodies[body] = body_obj
        
        return cls(name, bodies)

    @classmethod
    def create_from_eph_data(cls, start_time, eph_kernel_path, yaml_path):
        eph_kernel = SPK.open(eph_kernel_path)
        time = Time(start_time).jd
        yaml_dict = parse_yaml(yaml_path)
        name = yaml_dict['name']
        bodies = {}
        
        for body in yaml_dict['bodies']:
            properties = yaml_dict['bodies'][body]
            position = np.zeros(3)
            velocity = np.zeros(3)
            if properties.get('enum', None):
                body_enum = properties['enum']
                position, velocity = \
                    eph_kernel[0, body_enum].compute_and_differentiate(time)
                position = position*1000
                velocity = velocity*(1000/(24*3600))

            mass = properties['mass']
            pos = position
            vel = velocity
            parent = bodies.get(properties.get('parent'), None)
            radius = properties.get('radius', 0.1)

            state = State(pos, vel)
            body_obj = OrbitalBody(body, mass, state, parent_body=parent, radius=radius)
            body_obj.enum = properties['enum']
            bodies[body] = body_obj
        
        return cls(name, bodies)



if __name__ == '__main__':
    test_state_1 = State()
    test_state_2 = State(pos=5*np.ones(3))
    parent_body = OrbitalBody('test_parent', 200.0, test_state_1)
    test_body = OrbitalBody('testo_9', 100.0, test_state_2, parent_body=parent_body)
    print(test_body.force_rel(parent_body))
    test_sys = OrbitalSystem('test_sol', [parent_body, test_body])
    test_sys.apply_accs()
    print(test_body.acc_n)
    print(parent_body.acc_n)