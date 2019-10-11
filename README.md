# g2h

Is kind of graph viewer in a terminal.

## Get started

Nowadays, when have been developed a cli prompt part, `g2h` works with stdin as a major input system.

If you write such commands like this you will get such a type of output.

```text
>>> edge add hello
>>> edge add world
>>> edge add g2h    
>>> edge connect 0 2 
>>> edge connect 0 1
>>> edge connect 1 2
>>> print
 -------------------       
|                   |
| --------          |      
||        |         |     
||        | --------|      
||        ||        ||  
--------- --------- ------- 
|       | |       | |     | 
| hello | | world | | g2h | 
|       | |       | |     | 
--------- --------- -------
```

## Commands

| type | command | effect |
|:----:|:-------:|:------|
| edge |   add   | get message and place it as a new edge |
| edge |   connect   | takes 2 parametes, indexes which edges we whant to have connected |
| |   print   | print, graph which was built |

## Roadmap

- [ ] Draw verticales below nodes list
- [x] Support more then len(node_data) connections on node, encrese it's scope
- [x] A Dinamic setting on space on connection
- [x] CLI Promt
- [ ] Support a history of commands(Up/Down buttons as press buttons actions)
- [ ] Search a way on the graph
- [x] Connector types (for related graphs)
- [x] Refactoring print method
- [ ] Refactoring
