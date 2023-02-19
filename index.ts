import fs from "fs";

const ipsURL = "https://github.com/mat-1/minecraft-scans/blob/main/ips?raw=true";

const debug = false;

class Address {
    _address: string;
    _port: number;

    constructor(address: string, port: number) {
        let addr = address.split(".").map(Number);
        if (addr.length !== 4) throw new Error(`Bad length '${addr.length}'`);
        addr.forEach(num => { if (num > 255) throw new Error(`Number '${num}' is greater than 255`); });
        this._address = address;
        this._port = port;
    }

    get address(): string { return this._address; }
    get port(): number { return this._port; }
};


async function run() {

    let ips: Address[] = [];

    let out = Buffer.from(await (await fetch(ipsURL)).arrayBuffer());

    // fs.writeFileSync("ips.bin", out);

    let length = out.byteLength / 6;

    function readNext(): Address {
        let address = out.readUInt8().toString();
        for (let i = 0; i < 3; i++) address += "." + out.readUInt8(i + 1);
        let port = out.readUInt16BE(4);
        out = out.subarray(6);
        if (debug) console.log(`Byte length - OLD: ${length} | NEW: ${out.byteLength / 6}`);
        length = out.byteLength / 6;
        return new Address(address, port);
    }

    while (out.length > 0) ips.push(readNext());

    fs.writeFileSync("ips_plaintext.txt", ips.map(addr => `${addr.address}:${addr.port}`).join("\n"));

}

run();
