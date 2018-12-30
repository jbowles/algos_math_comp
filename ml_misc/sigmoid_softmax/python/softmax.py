"""
    sigmoid and softmax
    https://nolanbconaway.github.io/blog/2017/softmax-numpy
"""
import numpy as np


def softmax(X, theta=1.0, axis=None):
    """
    Compute softmax for each elem along axis

    params
    ------
    X: ND-Array, floats
    theta: float; used to multiply prio exponentiation
    axis: axis to comput values; defaults to first non-singleton axis
    """
    # at least 2D
    y = np.atleast_2d(X)
    if axis is None:
        axis = next(j[0] for j in enumerate(y.shape) if j[1] > 1)
    # scale by theta parameter
    y = y * float(theta)
    # sub max for numerical stability
    y = y - np.expand_dims(np.max(y, axis=axis), axis)
    # exponentiate
    y = np.exp(y)
    # take sum along axis
    ax_sum = np.expand_dims(np.sum(y, axis=axis), axis)
    # divide elem-wise
    p = y / ax_sum
    # if Xas 1D flatten back
    if len(X.shape) == 1:
        p = p.flatten()
    return p


scores4 = np.array([1.1, 5.0, 2.8, 7.3])
scores4x4 = np.array([
    [1.0, 2.0, 3.0, 6.0],
    [2.0, 4.0, 5.0, 6.0],
    [3.0, 8.0, 7.0, 6.0],
    [3.0, 8.0, 7.0, 6.0],
])
scores3x4 = np.array([
    [1.1, 5.0, 2.2, 7.3],
    [6.5, 3.2, 8.8, 5.3],
    [2.7, 5.1, 9.6, 7.4],
])


X = np.array([
    [1.1, 5.0, 2.2, 7.3],
    [6.5, 3.2, 8.8, 5.3],
    [2.7, 5.1, 9.6, 7.4],
])

# softmax over rows
softmax(X, theta=0.5, axis=0)
# array([[0.05523253, 0.40686118, 0.01458578, 0.41336824],
#       [0.82184522, 0.16541741, 0.39545889, 0.15206968],
#       [0.12292225, 0.4277214 , 0.58995534, 0.43456208]])

# softmax over columns
softmax(X, theta=0.5, axis=1)
# array([[0.03128922, 0.21992213, 0.05423213, 0.69455652],
#       [0.20412102, 0.03920142, 0.64465356, 0.11202399],
#       [0.02159544, 0.07169938, 0.68026473, 0.22644046]])

# softmax over columns, and squash it!
softmax(X, theta=50.0, axis=1)
# array([[2.33727929e-135, 1.13797987e-050, 1.79848622e-111,
#        1.00000000e+000],
#       [1.13797987e-050, 2.49772757e-122, 1.00000000e+000,
#        9.96473301e-077],
#       [1.47368188e-150, 1.92194773e-098, 1.00000000e+000,
#        1.68891188e-048]])


def f(x):
    """ naive version of softmax, not theta scale... no numerical stability"""
    return np.exp(x) / np.sum(np.exp(x), axis=0)


def softmax_theta(x, theta=1.0):
    """ naive version of softmax... no numerical stability"""
    t = np.exp(x * theta)
    t /= np.sum(t, axis=0)
    return t
