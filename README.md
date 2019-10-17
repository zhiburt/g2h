# g2h

Is kind of graph viewer in a terminal.

## Get started

Nowadays, when have been developed a cli prompt part, `g2h` works with stdin as a major input system.

If you write such commands like this you will get such a type of output.

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

## Commands

| type | command | effect |
|:----:|:-------:|:------|
| |   print   | print, graph which was built |
| edge |   add   | get message and place it as a new edge |
| edge |   connect   | takes 2 parametes, indexes which edges we whant to have connected |
| settings |   gap edge   | takes size of gap between edges |
| settings |   gap verticales   | takes size of gap between connection lines |

## Roadmap

- [ ] Another type of view of graph
- [ ] Switch between views
- [ ] Draw verticales below nodes list
- [x] Support more then len(node_data) connections on node, encrese it's scope
- [x] A Dinamic setting on space on connection
- [x] CLI Promt
- [ ] Support a history of commands(Up/Down buttons as press buttons actions)
- [ ] Search a way on the graph
- [x] Connector types (for related graphs)
- [x] Refactoring print method
- [ ] Refactoring
