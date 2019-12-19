console.log("first js program");
var fs = require('fs');
var user = require('./js/user_data_types.js');

fs.readFile('src\\main.rs','utf-8',function(err,data){
	if (err){ console.log(err);}
	else{console.log(data)}
});

var a1=[1,4,9,16];
var a2=a1.map(Math.sqrt);
console.log(a2);
console.log(user.user.name);