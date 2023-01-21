import numpy as np


class ExplicitRKSolver:
    k = {}
    s = {}

    def __init__(self, system):
        self.system = system

    def iterate(self, d_t, data_handler):
        for idx in self.k:
            self.system.apply_accs()

            for body in self.system.bodies.values():
                x = body.state
                
                x.vel_k += self.s[idx]*x.vel_n
                x.acc_k += self.s[idx]*x.acc_n

                if idx != list(self.k.keys())[-1]:
                    x.pos_n = x.pos_i + d_t*x.vel_n * self.k[idx + 1]
                    x.vel_n = x.vel_i + d_t*x.acc_n * self.k[idx + 1]
                else: 
                    x.pos_i = x.pos_i + d_t*(x.vel_k)
                    x.vel_i = x.vel_i + d_t*(x.acc_k)

                    x.pos_n = x.pos_i
                    x.vel_n = x.vel_i

                    x.vel_k = np.array([0.0, 0.0, 0.0])
                    x.acc_k = np.array([0.0, 0.0, 0.0])

                    data_handler.store_data(body)


class RungeKutta4Solver(ExplicitRKSolver):

    k = {1: 1.0, 2: 0.5, 3: 0.5, 4: 1.0}
    s = {1: (1/6), 2: (1/3), 3: (1/3), 4: (1/6)}


class RalstonSolver(ExplicitRKSolver):

    k = {1: (2/3), 2: (2/3)}
    s = {1: (1/4), 2: (3/4)}


class ForwardEulerSolver(ExplicitRKSolver):

    k = {1: 1}
    s = {1: 1}