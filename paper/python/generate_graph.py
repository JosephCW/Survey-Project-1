import matplotlib.pyplot as plt

num_runs = [1, 5, 10, 50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 750, 1000, 1750, 2500, 3500]
forced = [0.000000033, 0.000000085, 0.000000448, 0.000030930, 0.000194367, 0.000594095, 0.001366055, 0.002607847, 0.004420136, 0.006950275, 0.010421218, 0.014655456, 0.020030729, 0.067125954, 0.157617270, 0.837418008, 2.432867191, 6.606255604]
kadane = [0.000000025, 0.000000036, 0.000000051, 0.000000125, 0.000000184, 0.000000260, 0.000000331, 0.000000402, 0.000000466, 0.000000573, 0.000000639, 0.000000716, 0.000000956, 0.000001231, 0.000001779, 0.000002682, 0.000003601, 0.000005064]

line_forced, = plt.plot(num_runs, forced, label='Brute Force Algorithm', linewidth=4) 
line_kadane, = plt.plot(num_runs, kadane, label='Kadane\'s Algorithm', linewidth=5)
plt.legend(handles=[line_forced, line_kadane])

# axis(xmin, xmax, ymin, ymax)
plt.axis([0, 2750, 0, 4])
plt.ylabel('Average Time in Seconds') 
plt.xlabel('Size of Array')
plt.title('Time to Calculate Maximum Sum in Array of Size N') 
#plt.show()
plt.savefig('avgTimeGraph.png')
