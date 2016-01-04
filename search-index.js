var searchIndex = {};
searchIndex['recycler'] = {"items":[[0,"","recycler","",null,null],[3,"TrashRecycler","","A \"recycler\" that doesn't recycle anything, instead just dropping anything\nit is given. This is particularly useful for primitive types such as `i32`\nthat do not have `Drop` implementations.",null,null],[3,"StringRecycler","","",null,null],[3,"VecRecycler","","",null,null],[12,"recycler","","",0,null],[3,"OptionRecycler","","",null,null],[12,"recycler","","",1,null],[5,"make_recycler","","",null,{"inputs":[],"output":{"name":"defaultrecycler"}}],[8,"Recyclable","","A value that has some default type that can recycle it.",null,null],[16,"DefaultRecycler","","",2,null],[8,"Recycler","","",null,null],[16,"Item","","",3,null],[10,"recycle","","",3,{"inputs":[{"name":"recycler"},{"name":"item"}],"output":null}],[10,"recreate","","",3,{"inputs":[{"name":"recycler"},{"name":"item"}],"output":{"name":"item"}}],[11,"default","","",4,{"inputs":[{"name":"trashrecycler"}],"output":{"name":"self"}}],[11,"recycle","","",4,{"inputs":[{"name":"trashrecycler"},{"name":"item"}],"output":null}],[11,"recreate","","",4,{"inputs":[{"name":"trashrecycler"},{"name":"item"}],"output":{"name":"item"}}],[11,"default","","",5,{"inputs":[{"name":"stringrecycler"}],"output":{"name":"stringrecycler"}}],[11,"recycle","","",5,{"inputs":[{"name":"stringrecycler"},{"name":"string"}],"output":null}],[11,"recreate","","",5,{"inputs":[{"name":"stringrecycler"},{"name":"string"}],"output":{"name":"string"}}],[11,"new","","",5,{"inputs":[{"name":"stringrecycler"}],"output":{"name":"string"}}],[11,"new_from","","",5,{"inputs":[{"name":"stringrecycler"},{"name":"str"}],"output":{"name":"string"}}],[11,"recycle","","",0,{"inputs":[{"name":"vecrecycler"},{"name":"vec"}],"output":null}],[11,"recreate","","",0,{"inputs":[{"name":"vecrecycler"},{"name":"vec"}],"output":{"name":"vec"}}],[11,"new","","",0,null],[11,"default","","",0,{"inputs":[{"name":"vecrecycler"}],"output":{"name":"self"}}],[11,"default","","",1,{"inputs":[{"name":"optionrecycler"}],"output":{"name":"optionrecycler"}}],[11,"recycle","","",1,{"inputs":[{"name":"optionrecycler"},{"name":"option"}],"output":null}],[11,"recreate","","",1,{"inputs":[{"name":"optionrecycler"},{"name":"option"}],"output":{"name":"option"}}],[11,"deref","","",1,{"inputs":[{"name":"optionrecycler"}],"output":{"name":"target"}}],[11,"deref_mut","","",1,{"inputs":[{"name":"optionrecycler"}],"output":{"name":"target"}}]],"paths":[[3,"VecRecycler"],[3,"OptionRecycler"],[8,"Recyclable"],[8,"Recycler"],[3,"TrashRecycler"],[3,"StringRecycler"]]};
initSearch(searchIndex);
