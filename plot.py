#!/usr/bin/env python

from matplotlib import pyplot as plt, gridspec
import numpy as np
import json

# for broken y axis
def plot(data):
    plt.xkcd()
    fig = plt.figure(constrained_layout=True, figsize=(10,6))
    spec = gridspec.GridSpec(7, 1, figure=fig)
    ax1 = fig.add_subplot(spec[0:5, 0])
    ax2 = fig.add_subplot(spec[5:, 0])

    ax1.step(range(len(data)), data)
    ax2.step(range(len(data)), data)

    # ax1.set_ylim(_, _)
    ax2.set_ylim(0, 100)

    ax1.spines['bottom'].set_visible(False)
    ax1.spines['right'].set_visible(False)
    ax2.spines['right'].set_visible(False)
    ax1.spines['top'].set_visible(False)
    ax2.spines['top'].set_visible(False)

    ax1.xaxis.set_visible(False)
    plt.xticks([0, 7776])

    # ax1.set_yticks([210000, 220000, 230000, 240000, 250000, 260000])
    # ax1.set_yticklabels(['210 KB', '220 KB', '230 KB', '240 KB', '250 KB', '260 KB'])
    ax2.set_yticks([])

    ax1.set_title('THIS LIB')
    plt.xlabel('SYMBOLS')
    plt.ylabel('ALLOCS')

    # begin broken axis indicators
    # ref: https://matplotlib.org/3.1.0/gallery/subplots_axes_and_figures/broken_axis.html

    d = 0.015
    kwargs = dict(transform=ax1.transAxes, color='k', clip_on=False)
    ax1.plot((-d, +d), (-d, +d), **kwargs)
    kwargs.update(transform=ax2.transAxes)
    ax2.plot((-d, +d), (1 - d, 1 + d), **kwargs)

    # end broken axis indicators

    plt.show()

# for continuous y axis
def plot(data):
    plt.xkcd()
    fig, ax = plt.subplots(1, 1, figsize=(10,6))

    ax.step(range(len(data)), data)

    ax.spines['right'].set_visible(False)
    ax.spines['top'].set_visible(False)

    plt.xticks([0, 7776])

    # ax.set_yticks([0, 50000, 100000, 150000, 200000, 250000, 300000])
    # ax.set_yticklabels(['0', '50 KB', '100 KB', '150 KB', '200 KB', '250 KB', '300 KB'])

    ax.set_title('THIS LIB')
    plt.xlabel('SYMBOLS')
    plt.ylabel('ALLOCS')

    plt.show()

def display(events):
    data = np.zeros(7777)
    allocated = 0
    i = 0
    for event in events:
        if event.get('Alloc'):
            allocated += event['Alloc']['size']
        elif event.get('Freed'):
            allocated -= event['Freed']['size']
        else:
            data[i] = allocated
            i += 1
    plot(data)

if __name__ == '__main__':
    with open('events.ldjson') as events:
        events = [json.loads(event) for event in events.readlines()]
        display(events)
