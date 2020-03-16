import Adapter from './Adapter';

class OilLevelAdapter {
    constructor() {
        this.baseAddress = `http://localhost:8120`;
        this.adapter = new Adapter();
        this.get = this.get.bind(this);
    }
    async get(id) {
        return this.adapter.get(this.baseAddress, id);
    }
}

export default OilLevelAdapter