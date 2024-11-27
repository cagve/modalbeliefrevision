import networkx as nx
import math
import matplotlib.pyplot as plt

# Data (Kripke models)
data = [
        {"W": ["pq"], "w": "pq"},
        {"W": ["q"], "w": "q"},
        {"W": ["∅"], "w": "∅"},
        {"W": ["p", "q"], "w": "p"},
        {"W": ["p", "q"], "w": "q"},
        {"W": ["p", "∅"], "w": "p"},
        {"W": ["p", "∅"], "w": "∅"},
        {"W": ["q", "pq"], "w": "q"},
        {"W": ["q", "pq"], "w": "pq"},
        {"W": ["q", "∅"], "w": "q"},
        {"W": ["q", "∅"], "w": "∅"},
        {"W": ["pq", "∅"], "w": "pq"},
        {"W": ["pq", "∅"], "w": "∅"},
        {"W": ["p", "q", "pq"], "w": "p"},
        {"W": ["p", "q", "pq"], "w": "q"},
        {"W": ["p", "q", "pq"], "w": "pq"},
        {"W": ["p", "q", "∅"], "w": "p"},
        {"W": ["p", "q", "∅"], "w": "q"},
        {"W": ["p", "q", "∅"], "w": "∅"},
        {"W": ["p", "pq", "∅"], "w": "p"},
        {"W": ["p", "pq", "∅"], "w": "pq"},
        {"W": ["p", "pq", "∅"], "w": "∅"},
        {"W": ["q", "pq", "∅"], "w": "q"},
        {"W": ["q", "pq", "∅"], "w": "pq"},
        {"W": ["q", "pq", "∅"], "w": "∅"},
        {"W": ["p", "q", "pq", "∅"], "w": "p"},
        {"W": ["p", "q", "pq", "∅"], "w": "q"},
        {"W": ["p", "q", "pq", "∅"], "w": "pq"},
        {"W": ["p", "q", "pq", "∅"], "w": "∅"},
]



def draw_pointed_models(data, cols):
    cols = 4
    rows = math.ceil(len(data) / cols)
    _, axes = plt.subplots(cols, rows, figsize=(20, 15))  
    axes = axes.flatten()  
    i = 0
    while i < len(axes):
        if i>len(data)-1:
            axes[i].set_xticks([])  # Remove x ticks
            axes[i].set_yticks([])  # Remove y ticks
            for spine in axes[i].spines.values():
                spine.set_edgecolor('black')  # Set the border color
                spine.set_linewidth(0)  # Set the border width
        else:
            model = data[i]
            worlds = model['W']
            actual_world = model['w']
            
            # Create a directed graph
            G = nx.DiGraph()

            G.add_nodes_from(worlds)
            pos = nx.kamada_kawai_layout(G)

            nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=2000,ax=axes[i])
            nx.draw_networkx_nodes(G, pos, nodelist=[actual_world], node_color='orange', node_size=2000,ax=axes[i])
            nx.draw_networkx_labels(G, pos, font_size=12, font_color='black', ax=axes[i])

            # Set the title of each subplot
            axes[i].set_title(f"Model {i + 1}: W = {worlds}, w = {actual_world}", fontsize=16)
            axes[i].set_xlim([-1.5, 1.5])
            axes[i].set_ylim([-1.5, 1.5])
            axes[i].set_xticks([])  # Remove x ticks
            axes[i].set_yticks([])  # Remove y ticks

            # Set a border around each subplot
            for spine in axes[i].spines.values():
                spine.set_edgecolor('black')  # Set the border color
                spine.set_linewidth(2)  # Set the border width
        i+=1

    plt.tight_layout()
    plt.show()


# Function to filter non-repeated models based only on W
def filter_unique_models(data):
    unique_W = []
    seen = set()  # Set to keep track of seen W tuples

    for model in data:
        # Create a frozenset of W for uniqueness (order doesn't matter)
        key = frozenset(model['W'])

        if key not in seen:
            seen.add(key)  # Mark this W combination as seen
            unique_W.append(sorted(model['W']))  # Add sorted W list to unique_W

    return unique_W

def draw_models(data,cols, dif=False, save=False):
    models = filter_unique_models(data)
    if not dif:
        cols = 4
        rows = math.ceil(len(models) / cols)
        _, axes = plt.subplots(cols, rows, figsize=(20, 15))  
        axes = axes.flatten()  
        i = 0
        while i < len(axes):
            if i>len(models)-1:
                axes[i].set_xticks([])  # Remove x ticks
                axes[i].set_yticks([])  # Remove y ticks
                for spine in axes[i].spines.values():
                    spine.set_edgecolor('black')  # Set the border color
                    spine.set_linewidth(0)  # Set the border width
            else:
                worlds = models[i]
                G = nx.DiGraph()
                G.add_nodes_from(worlds)
                pos = nx.kamada_kawai_layout(G)

                nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=2000,ax=axes[i])
                nx.draw_networkx_labels(G, pos, font_size=12, font_color='black', ax=axes[i])

                # Set the title of each subplot
                axes[i].set_title(f"Model {i + 1}: W = {worlds}", fontsize=16)
                axes[i].set_xlim([-1.5, 1.5])
                axes[i].set_ylim([-1.5, 1.5])
                axes[i].set_xticks([])  # Remove x ticks
                axes[i].set_yticks([])  # Remove y ticks

                # Set a border around each subplot
                for spine in axes[i].spines.values():
                    spine.set_edgecolor('black')  # Set the border color
                    spine.set_linewidth(2)  # Set the border width
            i+=1

        plt.tight_layout()
        if save:
            plt.savefig('models.png')
        else:
            plt.show()
    else:
        for i,worlds in enumerate(models):
            fig, ax = plt.subplots()
            G = nx.DiGraph()
            G.add_nodes_from(worlds)
            pos = nx.kamada_kawai_layout(G)

            nx.draw_networkx_nodes(G, pos, node_color='lightblue', node_size=2000, ax=ax)
            nx.draw_networkx_labels(G, pos, font_size=12, font_color='black', ax=ax)

            ax.set_title(f"Model {i + 1}: W = {worlds}", fontsize=16)
            ax.set_xlim([-1.5, 1.5])
            ax.set_ylim([-1.5, 1.5])
            ax.set_xticks([])  # Remove x ticks
            ax.set_yticks([])  # Remove y ticks

            # Set a border around each subplot
            for spine in ax.spines.values():
                spine.set_edgecolor('black')  # Set the border color
                spine.set_linewidth(2)  # Set the border width

            plt.tight_layout()
            if save:
                plt.savefig(f"model_{i+1}")
            else:
                plt.show()



draw_models(data,4,dif=True,save=True)
