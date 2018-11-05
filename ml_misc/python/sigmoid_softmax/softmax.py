"""
    sigmoid and softmax
    https://nolanbconaway.github.io/blog/2017/softmax-numpy
"""
import numpy as np


def softmax_naive(x):
    """ naive version of softmax does not scale the ... """
    return np.exp(x) / np.sum(np.exp(x), axis=0)
