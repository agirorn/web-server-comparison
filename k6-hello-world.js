import http from 'k6/http';
import { check } from 'k6';
// import { sleep } from 'k6';

export const options = {
  vus: 10,
  duration: '30s',
};
export default function () {
  // const res = http.get('http://localhost:3000/hello-world');
  const res = http.get('http://0.0.0.0:3000/hello-world');
  // sleep(1);
  check(res, { 'status was 200': (r) => r.status == 200 });
}

