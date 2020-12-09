import re

INPUT_FILE = "day7.input"
MAIN_REG = re.compile(r"([a-z ]*) bags? contain")
CONTAIN_REG = re.compile(r"(\d+) ([a-z ]+) bags?")


class Graph:
    def __init__(self):
        self.vertices = []
        self.edges = {}

    def add_vertex(self, vnum):
        self.vertices.append(vnum)

    def add_edge(self, fromnum, tonum, weight):
        if not (fromnum in self.edges):
            self.edges[fromnum] = []
            
        self.edges[fromnum].append((tonum, weight))

    def dfs(self, start_vert):
        discovered = []
        stack = []
        stack.append(start_vert)

        while len(stack) != 0:
            v = stack.pop()

            if v not in discovered:
                discovered.append(v)
                if v not in self.edges:
                    continue
                for dest_vert, weight in self.edges[v]:
                    stack.append(dest_vert)

        return discovered

    def get_bag_weight(self, bag):
        res = 0

        if bag not in self.edges:
            return 0
        elif len(self.edges[bag]) == 0:
            return 0
        else:
            for other_bag, weight in self.edges[bag]:
                res += weight*self.get_bag_weight(other_bag) + weight

        return res


def main():
    g = Graph()
    g_rev = Graph()

    with open(INPUT_FILE) as f:
        lines = f.readlines()
        for line in lines:
            match1 = MAIN_REG.match(line)
            if match1 is None:
                print("ack!")

            main_name = match1.group(1)
            if main_name == 'shiny gold':
                print("SHINY GOLD: {}".format(hash(main_name)))

            contains = CONTAIN_REG.findall(line)
            if contains == []:
                #print("{} contains no other bags".format(main_name))
                pass

            main_hash = hash(main_name)
            g.add_vertex(main_hash)
            g_rev.add_vertex(main_hash)

            #contains_arr = []
            for c in contains:
                #contains_arr.append((int(c[0]), hash(c[1])))
                g.add_edge(main_hash, hash(c[1]), int(c[0]))
                g_rev.add_edge(hash(c[1]), main_hash, int(c[0]))

    
    print("num vertices: {}".format(len(g.vertices)))

    shiny_gold = hash("shiny gold")

    disc = g_rev.dfs(shiny_gold)
    print("P1: {}".format(len(disc)-1))
    print("P2: {}".format(g.get_bag_weight(shiny_gold)))
    


if __name__ == "__main__":
    main()
