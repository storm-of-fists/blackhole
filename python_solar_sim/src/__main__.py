from solver import (
    RungeKutta4Solver,
    RalstonSolver,
    ForwardEulerSolver
)
from utilities import (
    DataHandler,
)
from progress.bar import IncrementalBar
import system
from constants import (
    EPHEMERIS_PATH,
    YAML_PATH
)
from jplephem.spk import SPK
from astropy.time import Time


t_n = 0.0
t_f = 1e8
d_t = 1e5

start_time = "2021-07-30 00:00"

end_t = Time(start_time).jd + (t_f/(3600*24))

data = DataHandler()
orbital_system = system.OrbitalSystem.create_from_eph_data(start_time, 
                                                           EPHEMERIS_PATH,
                                                           YAML_PATH / "sol_system.yaml")
solver = RungeKutta4Solver(orbital_system)


class Simulation:

    def __init__(self, system, solver, data_handler):
        self.system = system
        self.solver = solver
        self.data_handler = data_handler

    def run_system(self, t_n, t_f, t_step):
        
        progress_bar = IncrementalBar('Sim Status', 
                                      max=(t_f-t_n)/t_step,
                                      suffix='%(percent)d%%')

        while t_n < t_f:
            self.solver.iterate(t_step, self.data_handler)
            t_n += t_step
            progress_bar.next()


    def write_data(self):
        for body in self.system.bodies.values():
            self.data_handler.write_data(body)
    
    def check_eror(self, end_time):
        eph_kernel = SPK.open(EPHEMERIS_PATH)
        for body in [b for b in self.system.bodies.values() if b.enum != 0]:
            eph_pos = eph_kernel[0, body.enum].compute(end_time)*1000
            print(f'Error pos for {body.name}: {eph_pos - body.state.pos_i}')


def main():
    sim = Simulation(orbital_system, solver, data)
    sim.run_system(t_n, t_f, d_t)
    sim.write_data()
    sim.check_eror(end_t)


if __name__=='__main__':
    main()
    








