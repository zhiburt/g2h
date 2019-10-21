# g2h

Is kind of graph viewer in a terminal.

## Get started

Nowadays, when have been developed a cli prompt part, `g2h` works with stdin as a major input system.
The project was started developing with such a representation of graph as this one. You can find how to create it at [the simple graph example](#simple-graph).

```text
 ----------                                                    
|          |                                                   
|  -------------------                                         
| |        |          |                                        
| |  -------------------                                       
| | |      |          | |                                      
| | |      |          | |       ----------                     
| | |      |          | |      |          |                    
| | |      |          | |      |  --------------------         
| | |      |          | |      | |        |           |        
| | |  -------------------------------------------------       
| | v v    v          v |      | |        v           v |      
---------  ---------  -------  ---------  ----------  ---------  
|       |  |       |  |     |  |       |  |        |  |       |  
| hello |  | world |  | g2h |  | macha |  | andrey |  | vadim |  
|       |  |       |  |     |  |       |  |        |  |       |  
---------  ---------  -------  ---------  ----------  ---------
```

A few time after was begun developing matrix type. Which supports path find algorithm, currently only Dijkstra's.
You can find this example at [the example section](#simple-matrix).

![Demo Animation](../demos/matrix.png?raw=true)



## Commands

| type | command | effect |
|:----:|:-------:|:------|
| |   print   | print, graph which was built |
| edge |   add   | get message and place it as a new edge |
| edge |   connect   | takes 2 parametes, indexes which edges we whant to have connected |
| matrix |   | takes 2 parametes size of matrix, width and hight  |
| matrix |   search   | takes 2 parametes, start point and end point |
| matrix |   block   | takes index of node which is removed all links |
| settings |   gap edge   | takes size of gap between edges |
| settings |   gap verticales   | takes size of gap between connection lines |

## Examples

The examples might be a bit outdated.

### Simple graph

```
>>> edge add hello
>>> edge add world
>>> edge add g2h
>>> edge add macha
>>> edge add andrey
>>> edge add vadim
>>> edge connect 0 1
>>> edge connect 0 2
>>> edge connect 2 0
>>> edge connect 3 4
>>> edge connect 3 5 
>>> edge connect 5 0
>>> print
```

### Simple matrix

```
>>> matrix init 10 10              
>>> matrix search 65 99
```

## Roadmap

- [x] An another type of view
- [ ] Support a way to add a further nodes at the edges of matrix
- [ ] Mark blocked nodes(What exactly is a blocked node?)
- [ ] Switch between views
- [ ] Draw verticales below nodes list
- [x] Support more then len(node_data) connections on node, encrese it's scope
- [x] A Dinamic setting on space on connection
- [x] CLI Promt
- [ ] Support a history of commands(Up/Down buttons as press buttons actions)
- [x] Find a path on the graph
- [ ] Create a further bunch of search algorithms
- [x] Connector types (for related graphs)
- [x] Refactoring print method
- [ ] Refactoring of handling method
- [ ] Refactoring
