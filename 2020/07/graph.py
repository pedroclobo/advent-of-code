class Graph:

	def __init__(self, nodes, values=None):
		self.nodes = list(nodes)
		self.adj_list = {node: set() for node in self.nodes}
		self.values = values

	def add_edge(self, node1, node2, weight):
		self.adj_list[node1].add((node2, weight))

	def __str__(self):
		res = ''

		for key in self.adj_list.keys():
			res += str(key) + ": " + str(self.adj_list[key]) + '\n'

		return res

	def part1(self, main_bag):
		global count
		global visited

		count = 0
		visited = {}

		def bags_that_contain(bag):
			global count

			for (up_bag, _) in self.adj_list[bag]:
				if up_bag not in visited:
					bags_that_contain(up_bag)
					count += 1

				visited[up_bag] = True

		bags_that_contain(main_bag)

		return count

	def part2(self, main_bag):

		def bags_inside(bag):
			if len(self.adj_list[bag]) == 0:
				return 1

			count = 0
			for (sub_bag, value) in self.adj_list[bag]:
				count += value * bags_inside(sub_bag)

			return count + 1 if bag != main_bag else count

		return bags_inside(main_bag)
