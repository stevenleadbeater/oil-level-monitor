import axios from 'axios';

class Adapter {
    async getAll(url) {
        console.log(`GET: ${url}`);
        return axios.get(`${url}`);
    }
    async get(url, id) {
        console.log(`GET: ${url}/${id}`);
        return axios.get(`${url}/${id}`);
    }
    async post(url, payload) {
        console.log(`POST: ${url}, payload: ${JSON.stringify(payload)}`);
        return axios.post(url, payload);
    }
    async put(url, id, payload) {
        console.log(`PUT: ${url}/${id}, payload: ${JSON.stringify(payload)}`);
        return axios.put(`${url}/${id}`, payload);
    }
    async delete(url, id) {
        console.log(`DELETE: ${url}/${id}`);
        return axios.delete(`${url}/${id}`);
    }
}

export default Adapter