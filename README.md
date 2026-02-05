# Mazer

Draws mazes.

I'm learning Rust.

# Why Did You Do This?

I wanted to experiment with graph theory in Rust: a language that seems
allergic to complex graph models.

## The Problem with C

C is known for having manual memory management, which, while it's
easy to understand conceptually, can get you into subtle problems in
practice. And the consequences for small errors can be catastrophic,
especially in older desktop systems and newer embedded systems which
lack runtime checks on your dynamic memory allocation.

However, that's not entirely true. C _does_ have built-in,
hightly-efficient memory management... as long as you know the size of
the memory you need at compile time. That is to say, if you can declare
a variable locally within a stack frame, it's generally safe (as long as
you don't do something stupid) to pass a pointer to that local variable
_down_ the stack. _NEVER_ up the stack! If you do that, the C compiler
injects code that makes sure that when your function is returned, all
that allocated memory is returned to the runtime's cache (the stack).

It gets more complicated if you don't know the size of the memory you
need at compile time, but if you know the size you need at _runtime_,
it's not much more complicated. You need to use `malloc` and `free` to
request memory from the operating system and manage its lifecycle, but
if you give it the same lifecycle as a local variable, In that case, the
issue is a bit tedious, but trivial.

However, if you don't know the size of the memory you need even at
runtime---because you may need to add more or free some over the life
of your program---things get more interesting. At this point, you need
to create _data structures_ to manage your memory. Data structures are
a problem because they don't have any inherent ability to respond to
events and execute instructions. In order to properly manage dynamic
memory with data structures, you need to know when a variable goes
out of scope and which executable code to run. There are a few ways
to tackle this. (I've thought a lot about stack-based closures, and
functions with dynamic frames.) One simple way that is available to
C programmers is "methods". C is obviously not an "object-oriented"
language, and doesn't have any native concept of "methods", so I define
"methods" as being a function that is scoped to a data type. In C,
this scoping is all done manually with conventions. For example, a
method "foo" on a type "Widget" will be named `widget_foo()` and take
a `*Widget` as its first argument. There's also no way to get the C
runtime to automatically call your destructor method when a variable
goes out of scope. But if you adopt a convention such as:

```c Widget *widget_new() { Widget *w = malloc(sizeof(Widget)); //
initialize w return w; }

Widget *widget_delete(Widget *w) { if (w == NULL) { return NULL; } //
free any memory owned by w free(w); return NULL; } ```

Then, you can call `widget_new()` and `w = widget_delete(w)` in place
of `malloc` and `free`. After that, it becomes as tedious-but-trivial
as managing dynamic memory with the same lifecycle as local variables
to manage these data structures that "own" their own memory _PROVIDED
THAT_ the memory ownership graph is a directed-acyclic-graph (DAG). That
is, provided that every node in the graph, however many children it may
have, has only a SINGLE parent, except for a root node with no parent,
and that there are no cycles in the graph. A linked-list is the simplest
form of data structure with this DAG property, but anything up to a
complex tree graph will work.

However, programming is writing models that describe real-world systems,
and not every real-world system is a nice, well-behaved DAG. If the
real-world system you're attempting to model has a complex, undirected,
cyclical graph with orphaned sub-graphs, then you need the ability to
model that in code. The answer here becomes: you don't _need_ to use
your memory ownership graph as the basis for your model graph. It's
convenient if your model is a DAG, but if your model is not a DAG,
you need to use separate mechanisms for your memory ownership and to
model the relationships in your system. In this case, you need a memory
manager that will actually own all the nodes in your graph and have
responsibility for cleaning them up.

## The Go Solution

In Go, the solution is very much to keep the memory ownership graph
separate from the model graph. However, Go hides this from you. The
runtime treats all the memory relationships the developer creates as its
model, and the Garbage Collector is the REAL owner. In that way, the
memory ownership is a wide, flat tree, and definitely a DAG. Because
the graph you create as a developer is not the REAL memory ownership
graph, the Garbage Collector can use its fancy three-color algorithm to
walk the connections you create and find orphaned subgraphs to free. For
a lot of cases, this is a great solution, as it solves the C problem
by relieving the developer from the burden of having to manage that
memory. It does come with some drawbacks. For one, it adds a hefty
runtime to every program. (Not nearly as big as Java, Node, Python or
Ruby; but bigger than C). For another, the Garbage Collector wakes up
on its own schedule, which makes the performance of your app difficult
to predict. It also relies on the operating system for a lot, including
timers and multi-processing. It's therefore unsuitable for _programming_
operating-system-level things like timers and multi-processing systems.

## The Rust Solution

Rust just forbids you to make a memory model that is not a DAG. The
syntax requires the developer to specify at every point where a change
to the memory graph might be indicated whether or not the change is
intended. Changes that create orphans trigger the compiler to insert
runtime code to clean up those orphaned nodes. Changes that create
cycles are forbidden.

Unfortunately, although the compiler asks one question, and there are
only two possible answers, and although one answer is going to be the
correct answer 90% of the time (no change intended), the syntax of
Rust presents a new way of answering that question for each-and-every
scenario when it comes up, and almost never assigns the most ergonomic
way of answering to the obvious default. For example, when passing a
variable to a function, although in _almost every case_ you want to
retain ownership of the memory, Rust requires extra syntax (`&`) in
order to indicate that, and the default syntax is to transfer ownership
down the stack.

Rust also seems allergic to the idea of creating non-DAG graphs to model
non-DAG systems. The Rust devs have published a whole book about how
linked-lists are almost always a bad idea, and how to use `Vec` instead.
A linked-list is a DAG, and so it _is_ possible to make a linked-list
in Rust, but it's difficult to do correctly. Using a `Vec` as a memory
manager is a valid approach, but in the end, the indices into your `Vec`
become ersatz pointers, and Rust forces you to do your own, manual
memory management after all.

Another approach would be to use `Rc` and `rc::Weak` references in your
model.

It seems to me to be a major flaw in a language that it makes it
difficult to model potentially messy graphs that might exist in the real
world.

## Mazer as Graph Experiment

Having written my diatribe against Rust, I wanted to actually try it
out to see if it's as bad in practice as it seems in theory. The Mazer
program is a graph-based approach to drawing a solvable maze. The idea
is that I take the space of the maze and represent it as a highly
cyclical, undirected graph. I then walk the edges and prune them as I
go to convert it to a DAG, which can then be rendered graphically as a
maze. I use the `Vec` as memory manager approach, where indices into the
`Vec` are ersatz pointers.

The result of the first version was satisfying, but a little trivial
in practice. By blurring your vision a bit, you could see that all
paths tended to point to the "root" of the DAG, and, knowing that, the
solution of even large mazes was too easy to find.

In theory, the maze doesn't need to be rectangular and doesn't need
to be 2D. So in this version, I start with multiple roots and create
multiple "submaps" or "zones". These zones, in turn, can be solved using
the same algorithm to create unique paths. Then, we find all the borders
and open gates through each border. This creates a maze that doesn't
have any clear "center", and long paths are often forced to meander
around the 2D space.

This version also removes the custom web server. It uses Trunk to compile
the code to a WASM library, and the Dockerfile packages that in an NGinx
server. This library could be hosted as static assets as well.

## What Mazer does well

Mazer creates a rectangular, 2-dimensional graph in which there is
exactly one path between any two cells in the grid. Furthermore, it can
find that unique path pretty efficiently. It seems to be pretty fast,
although the SVGs it produces are complex, and the browsers sometimes
struggle with them when they get large.

## How Mazer can be improved

Mazer does not have any way to target paths of any particular
difficulty. The current implementation calculates a difficulty level
based on solution path length. Most mazes fall into the "Easy" or "Very
Easy" categories. "Medium" mazes occur about a third of the time.
"Hard" puzzles are somewhat rare, and "Very Hard" mazes are extremely
rare. (This logarithmic pattern comes in large part from the score
calculation, not anything intrinsically logarithmic in the algorithm.)

It would be nice to be able to target a minimum path length, including
on the meta-level to make sure the path passes through some minimum
number of zones.

In the current layout of the static assets, the Rust/WASM code renders
the maze with all of the hint and solution data embedded in the SVG,
which is returned as a string, and can be rather large. These elements
are turned on and off with JavaScript, which manipulates classes and an
externally-provided stylesheet. The next version will likely use Leptos
to render the SVG directly onto the DOM, and create/delete the hint
elements as-needed with bound functions. The more of mazer that can
be encapsulated in the WASM library, the more it can be embedded in
the HTML of an external site (say [mine](https://www.4zb.org)).

## Questions to Answer

There seems to be a sweet-spot for the number of zones. Too many
zones seems to cause the solution paths to become linear (as in
"as-the-crow-flies"). I don't know what that sweet-spot is (I currently
set it to 6 on a 60x20 map.)

Since the concrete maze can be abstacted into a meta-map, there's no
reason we couldn't abstract the meta-map into further meta layers. I
don't know what the effect of that might be in practical, puzzle-solving
terms. I want to try to make the levels of abstraction configurable
at run-time. Maybe as an array of root counts: `[4, 3, 2, 1]`
