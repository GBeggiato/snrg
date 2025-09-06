import matplotlib.pyplot as plt
import numpy as np

plt.hist(np.loadtxt("norm.txt"), bins=100)
plt.show()
