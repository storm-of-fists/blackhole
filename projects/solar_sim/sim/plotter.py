import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
import csv
import random
from constants import (
    DIR_PATH,
)


def plot_body_motion(bodies):

    x = []
    y = []

    n = 1
    
    for body in bodies:
        file = DIR_PATH / 'output' / f'{body}.csv'
        with open(file, 'r') as csv_file:
            plots = csv.reader(csv_file, delimiter=',')
            for row in plots:
                x.append(float(row[0]))
                y.append(float(row[1]))
            r = random.random()
            g = random.random()
            b = random.random()
            color = [r, g, b]
            plt.scatter(x[::n], y[::n], color=color)
            x = []
            y = []
        csv_file.close()

    plt.title('Interesting Graph\nCheck it out')
    plt.gca().set_aspect('equal', adjustable='box')
    plt.legend()
    plt.show()

def plot_3d_body_motion(bodies):
    
    x = []
    y = []
    z = []

    n = 200
    
    fig = plt.figure()
    ax = plt.axes(projection='3d')
    ax.view_init(45, 45)

    for body in bodies:
        file = DIR_PATH / 'output' / f'{body}.csv'
        with open(file, 'r') as csv_file:
            plots = csv.reader(csv_file, delimiter=',')
            for row in plots:
                x.append(float(row[0]))
                y.append(float(row[1]))
                z.append(float(row[2]))
            r = random.random()
            g = random.random()
            b = random.random()
            color = [r, g, b]
            ax.scatter3D(x, y, z, color=color);
            x = []
            y = []
            z = []
        csv_file.close()

    plt.title('Interesting Graph\nCheck it out')
    plt.legend()
    plt.show()

# def animate():
