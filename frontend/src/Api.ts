import Axios from "axios";

const ApiClient = Axios.create({
  baseURL: process.env.REACT_APP_API_URL,
  timeout: 10000,
  withCredentials: true,
});

export default ApiClient;
