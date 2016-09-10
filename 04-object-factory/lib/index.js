const addon = require('../native');
const o1 = addon.factory('hello');
const o2 = addon.factory('world');
console.log(`${o1.msg} ${o2.msg}`);
