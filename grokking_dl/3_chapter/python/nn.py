import numpy as np


def neural_network(input, weight):
    prediction = input * weight
    return prediction


def version1():
    weight = 0.1
    number_of_toes = [8.5, 9.5, 10, 9]
    input = number_of_toes[0]
    pred = neural_network(input, weight)
    print(pred)


def neural_network_n(inputs, weights):
    pred = w_sum(inputs, weights)
    return pred


def w_sum(a, b):
    assert(len(a) == len(b))
    output = 0
    for i in range(len(a)):
        output += (a[i] * b[i])
    return output


def version2():
    toes = [8.5, 9.5, 9.9, 9.0]
    wlrec = [0.65, 0.8, 0.8, 0.9]
    nfans = [1.2, 1.3, 0.5, 1.0]
    weights = [0.1, 0.2, 0.0]
    input = [toes[0], wlrec[0], nfans[0]]
    pred = neural_network_n(input, weights)
    print(pred)


def neural_network_dot(input, weights):
    pred = input.dot(weights)
    return pred


def version3():
    toes = np.array([8.5, 9.5, 9.9, 9.0])
    wlrec = np.array([0.65, 0.8, 0.8, 0.9])
    nfans = np.array([1.2, 1.3, 0.5, 1.0])
    weights = np.array([0.1, 0.2, 0.0])
    input = np.array([toes[0], wlrec[0], nfans[0]])
    pred = neural_network_dot(input, weights)
    print(pred)


def neural_network_ele_for(number, vector):
    output = [0, 0, 0]
    for i in range(len(vector)):
        output[i] = number * vector[i]
    return output


def ele_mul(number, vector):
    output = [0, 0, 0]
    for i in range(len(vector)):
        output[i] = number * vector[i]
    return output


def neural_network_ele_mul(input, weights):
    pred = ele_mul(input, weights)
    return pred


def version4():
    pred = neural_network_ele_mul(0.65, [0.3, 0.2, 0.9])
    print(pred)


def vect_mat_mul(vect, matrix):
    assert(len(vect) == len(matrix))
    # want 3 preditions
    output = [0, 0, 0]
    for i in range(len(vect)):
        output[i] = w_sum(vect, matrix[i])
    return output


def neural_network_vect_mat_mul(input, weights):
    pred = vect_mat_mul(input, weights)
    return pred


def version5():
    toes = [8.5, 9.5, 9.9, 9.0]
    winlossrec = [0.65, 0.8, 0.8, 0.9]
    nfans = [1.2, 1.3, 0.5, 1.0]
    input = [toes[0], winlossrec[0], nfans[0]]
    weights = [
        [0.1, 0.1, -0.3],  # hurt?
        [0.1, 0.2, 0.0],   # win?
        [0.0, 1.3, 0.1],   # sad?
    ]
    pred = neural_network_vect_mat_mul(input, weights)
    print(pred)


def net_mat_mul6(input):
    ih_wgt = [
        [0.1, 0.2, -0.1],
        [-0.1, 0.1, 0.9],
        [0.1, 0.4, -0.1]
    ]
    hp_wgt = [
        [0.3, 1.1, -0.3],
        [0.1, 0.2, 0.0],
        [0.0, 1.3, 0.1]
    ]
    weights = [ih_wgt, hp_wgt]
    hid = vect_mat_mul(input, weights[0])
    pred = vect_mat_mul(hid, weights[1])
    return pred


def version6_mat_mul():
    toes = [8.5, 9.5, 9.9, 9.0]
    winlossrec = [0.65, 0.8, 0.8, 0.9]
    nfans = [1.2, 1.3, 0.5, 1.0]
    input = [toes[0], winlossrec[0], nfans[0]]
    pred = net_mat_mul6(input)
    print(pred)


def weights_net_np6():
    ih_wgt = np.array([
        [0.1, 0.2, -0.1],
        [-0.1, 0.1, 0.9],
        [0.1, 0.4, -0.1]
    ]).T
    hp_wgt = np.array([
        [0.3, 1.1, -0.3],
        [0.1, 0.2, 0.0],
        [0.0, 1.3, 0.1]
    ]).T
    return [ih_wgt, hp_wgt]


def net_6(input, weights):
    hid = np.dot(input, weights[0])
    pred = np.dot(hid, weights[1])
    return pred


def version6():
    toes = [8.5, 9.5, 9.9, 9.0]
    winlossrec = [0.65, 0.8, 0.8, 0.9]
    nfans = [1.2, 1.3, 0.5, 1.0]
    input = [toes[0], winlossrec[0], nfans[0]]
    pred = net_6(input, weights_net_np6())
    print(pred)


version1()
version2()
version3()
version4()
version5()
version6_mat_mul()
version6()

"""
datasets is current status of the beginning of each game
for the first 4 games of season
    toes = current number of toes
    wlred = current games won (percent)
    nfans = fans count (in millions)
"""
