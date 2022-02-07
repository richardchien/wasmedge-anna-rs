import * as anna from "wasmedge_anna";

function ab2str(buf) {
    return String.fromCharCode.apply(null, new Uint8Array(buf));
}

function str2ab(str) {
    let buf = new ArrayBuffer(str.length);
    let bufView = new Uint8Array(buf);
    for (let i = 0, strLen = str.length; i < strLen; i++) {
        bufView[i] = str.charCodeAt(i);
    }
    return buf;
}

let value = "bar " + Math.random();
print("put result:", anna.put("foo", str2ab(value)));
let foo_val = anna.get("foo");
print("foo:", ab2str(foo_val)); // should be "bar " + a random number
let bar_val = anna.get("bar");
print("bar:", bar_val); // should be null
