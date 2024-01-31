# Mazer

Draws mazes.

I'm learning Rust. Here's how I understand it, philosophically:

## The web service.

If you just send a `GET` request to the root `/`, you will get a SVG of a 60x40 cell maze. A link at the top that says "link" will contain the unique URL of this map in case you want to bookmark it. At the bottom, a link called "solution" will re-render the map with the solution path illustrated with blue dots.

### Query Parameters

| Parameter | Type | Example | Effect |
| --------- | ---- | ------- | ------ |
| `width`   | integer | `60` | Sets the width (number of cells) of the maze. |
| `height`  | integer | `40` | Sets the height (number of cells) of the maze. |
| `seed`    | 32 bytes of Base64-encoded data | `KSIeZm4SKfkZrUcWVwx0Lm2GRgm_yvOCEIaWTgbPRgs%3D` | Sets the seed of the random number generator for recalling a previous map. (Note the height and width have to be the same as well.) |
| `solution` | any | `true` | Causes the solution to be displayed as blue dots. |

# Why Did You Do This?

I wanted to experiment with graph theory in Rust: a language that seems allergic to complex graph models.

## The Problem with C

C is known for having manual memory management, which, while it's easy to understand conceptually, can get you into subtle problems in practice. And the consequences for small errors can be catastrophic, especially in older desktop systems and newer embedded systems which lack runtime checks on your dynamic memory allocation.

However, that's not entirely true. C _does_ have built-in, hightly-efficient memory management... as long as you know the size of the memory you need at compile time. That is to say, if you can declare a variable locally within a stack frame, it's generally safe (as long as you don't do something stupid) to pass a pointer to that local variable _down_ the stack. _NEVER_ up the stack! If you do that, the C compiler injects code that makes sure that when your function is returned, all that allocated memory is returned to the runtime's cache (the stack).

It gets more complicated if you don't know the size of the memory you need at compile time, but if you know the size you need at _runtime_, it's not much more complicated. You need to use `malloc` and `free` to request memory from the operating system and manage its lifecycle, but if you give it the same lifecycle as a local variable, In that case, the issue is a bit tedious, but trivial.

However, if you don't know the size of the memory you need even at runtime---because you may need to add more or free some over the life of your program---things get more interesting. At this point, you need to create _data structures_ to manage your memory. Data structures are a problem because they don't have any inherent ability to respond to events and execute instructions. In order to properly manage dynamic memory with data structures, you need to know when a variable goes out of scope and which executable code to run. There are a few ways to tackle this. (I've thought a lot about stack-based closures, and functions with dynamic frames.) One simple way that is available to C programmers is "methods". C is obviously not an "object-oriented" language, and doesn't have any native concept of "methods", so I define "methods" as being a function that is scoped to a data type. In C, this scoping is all done manually with conventions. For example, a method "foo" on a type "Widget" will be named `widget_foo()` and take a `*Widget` as its first argument. There's also no way to get the C runtime to automatically call your destructor method when a variable goes out of scope. But if you adopt a convention such as:

```c
Widget *widget_new() {
    Widget *w = malloc(sizeof(Widget));
    // initialize w
    return w;
}

Widget *widget_delete(Widget *w) {
    if (w == NULL) {
        return NULL;
    }
    // free any memory owned by w
    free(w);
    return NULL;
}
```

Then, you can call `widget_new()` and `w = widget_delete(w)` in place of `malloc` and `free`. After that, it becomes as tedious-but-trivial as managing dynamic memory with the same lifecycle as local variables to manage these data structures that "own" their own memory _PROVIDED THAT_ the memory ownership graph is a directed-acyclic-graph (DAG). That is, provided that every node in the graph, however many children it may have, has only a SINGLE parent, except for a root node with no parent, and that there are no cycles in the graph. A linked-list is the simplest form of data structure with this DAG property, but anything up to a complex tree graph will work.

However, programming is writing models that describe real-world systems, and not every real-world system is a nice, well-behaved DAG. If the real-world system you're attempting to model has a complex, undirected, cyclical graph with orphaned sub-graphs, then you need the ability to model that in code. The answer here becomes: you don't _need_ to use your memory ownership graph as the basis for your model graph. It's convenient if your model is a DAG, but if your model is not a DAG, you need to use separate mechanisms for your memory ownership and to model the relationships in your system. In this case, you need a memory manager that will actually own all the nodes in your graph and have responsibility for cleaning them up.

## The Go Solution

In Go, the solution is very much to keep the memory ownership graph separate from the model graph. However, Go hides this from you. The runtime treats all the memory relationships the developer creates as its model, and the Garbage Collector is the REAL owner. In that way, the memory ownership is a wide, flat tree, and definitely a DAG. Because the graph you create as a developer is not the REAL memory ownership graph, the Garbage Collector can use its fancy three-color algorithm to walk the connections you create and find orphaned subgraphs to free. For a lot of cases, this is a great solution, as it solves the C problem by relieving the developer from the burden of having to manage that memory. It does come with some drawbacks. For one, it adds a hefty runtime to every program. (Not nearly as big as Java, Node, Python or Ruby; but bigger than C). For another, the Garbage Collector wakes up on its own schedule, which makes the performance of your app difficult to predict. It also relies on the operating system for a lot, including timers and multi-processing. It's therefore unsuitable for _programming_ operating-system-level things like timers and multi-processing systems.

## The Rust Solution

Rust just forbids you to make a memory model that is not a DAG. The syntax requires the developer to specify at every point where a change to the memory graph might be indicated whether or not the change is intended. Changes that create orphans trigger the compiler to insert runtime code to clean up those orphaned nodes. Changes that create cycles are forbidden.

Unfortunately, although the compiler asks one question, and there are only two possible answers, and although one answer is going to be the correct answer 90% of the time (no change intended), the syntax of Rust presents a new way of answering that question for each-and-every scenario when it comes up, and almost never assigns the most ergonomic way of answering to the obvious default. For example, when passing a variable to a function, although in _almost every case_ you want to retain ownership of the memory, Rust requires extra syntax (`&`) in order to indicate that, and the default syntax is to transfer ownership down the stack.

Rust also seems allergic to the idea of creating non-DAG graphs to model non-DAG systems. The Rust devs have published a whole book about how linked-lists are almost always a bad idea, and how to use `Vec` instead. A linked-list is a DAG, and so it _is_ possible to make a linked-list in Rust, but it's difficult to do correctly. Using a `Vec` as a memory manager is a valid approach, but in the end, the indices into your `Vec` become ersatz pointers, and Rust forces you to do your own, manual memory management after all.

Another approach would be to use `Rc` and `rc::Weak` references in your model.

It seems to me to be a major flaw in a language that it makes it difficult to model potentially messy graphs that might exist in the real world.

## Mazer as Graph Experiment

Having written my diatribe against Rust, I wanted to actually try it out to see if it's as bad in practice as it seems in theory. The Mazer program is a graph-based approach to drawing a solvable maze. The idea is that I take the space of the maze and represent it as a highly cyclical, undirected graph. I then walk the edges and prune them as I go to convert it to a DAG, which can then be rendered graphically as a maze. I use the `Vec` as memory manager approach, where indices into the `Vec` are ersatz pointers.

The result is a little messy. In theory, the maze didn't need to be rectangular and didn't need to be 2D, but my attempt to make my `Space` object generic exploded the complexity of the program.

## What Mazer does well

Mazer creates a rectangular, 2-dimensional graph in which there is exactly one path between any two cells in the grid. Furthermore, it can find that unique path pretty efficiently. It seems to be pretty fast, although the SVGs it produces are complex, and the browsers sometimes struggle with them when they get large.

## What Mazer does not do well

The mazes it makes are not challenging. Often times, they are trivial. Even when they are not trivial, you can often see the solution by standing a little ways away and blurring your vision a bit.

My attempt to make `Space` more generic by making it a Trait which could be implemented for, e.g. 3D spaces or hexagonal grids, while theoretically not complicating the algorithm in any way, created just a mess of complexity in the code.

## Future experiments for Mazer

### Implement `Deref` for ersatz pointers

A lot of the fiddly work I needed to do to get this memory-manager approach to work had to do with exchanging indices for references. I had problems, for example, looping over one collection and altering another, because both required a reference from the object that contained those collections. There were two approaches that worked: putting the logic into methods of the object that was being borrowed; and doing repeated lookups of the index in the `Vec` rather than holding on to a reference.

Moving the logic from one type to another is not always appropriate. `Automaton` logic should be on the `Automaton` object, not the `Graph` object.

I would like to explore the other solution: keeping the borrows as short as possible. I'd like to make it more ergonomic, however, and avoid writing long accessor chains like `self.graph.edges` each time. Leaning in to the "smart pointer" idea might be a way to do this. If my index type held a reference to its parent `Vec` and implemented `Deref` and `DerefMut`, I could perhaps keep those references around longer, but limit the actual borrow lifespans to just when they're needed.

### Make the mazes more difficult

This doesn't have anything to do with learning Rust or thinking about memory management. But I'm kind of disappointed that, although the mazes do everything I asked them to do, the result isn't as fun as I'd hoped. One idea is to make the maze with multiple roots and subgraphs. Since I can find a path from any point in a graph to any other point in a graph, I could link several graphs together and find a unique path through them all.

My first idea as to divide the maze space up into sections: basically to make a maze like I do now, but then put a smaller maze into each cell. That would result in clear lines that traverse the image, which is one thing the current mazes don't have. It would be visually apparent that the maze consisted of smaller mazes, each of which was fairly trivial to solve on its own.

The next idea is that it would be trivial to have the `Automaton` seed the maze with multiple roots, and let them grow organically to fill the space. This would avoid the clear partition lines, but there would be no guarantee that one subgraph would be reachable by another one. If a third subgraph extended a pseudopod between them to the edge, for example, it may not be possible to chart a path from a cell in the first subgraph to a cell in the second.

My current thinking (as of writing this) is that I allow multiple seeded roots to grow organically to fill the space, and whenever I close an edge that borders between two subgraphs, I catalog it. In the end, I will have a list of the subgraphs that do share a border. From there, I use a very similar edge-pruning algorithm to arrange my subgraphs conceptually into a DAG super-graph. Then, I can select one edge from each collection of border edges and open it. It becomes the start/end of the path within its two subgraphs. The existing algorithm for finding the path will then continue to work for finding a unique path between any two points in any two subgraphs, even if they do not directly share a border.



