import Adapter from './Adapter';

class OilLevelAdapter {
    constructor() {
        this.baseAddress = `http://${process.env.REACT_APP_BACKEND_HOST}:${process.env.REACT_APP_BACKEND_PORT}`;
        this.adapter = new Adapter();
        this.get = this.get.bind(this);
    }
    async get(id) {
        return this.adapter.getById(this.baseAddress, id);
    }
    async getHistory(id) {
        return this.adapter.get(`${this.baseAddress}/${id}/history`);
    }
}

export default OilLevelAdapter