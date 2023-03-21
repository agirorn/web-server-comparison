import http from 'k6/http';
import { check } from 'k6';
// import { sleep } from 'k6';

export const options = {
  vus: 400,
  duration: '30s',
};
export default function () {
  const res = http.post(
    'http://0.0.0.0:3000/save',
    // 'http://localhost:3000/save',
    '{"key":"value"}',
    {

      headers: {
        "Content-Type": "application/json",
      }
    }
  );
  // sleep(1);
  check(res, { 'status was 201': (r) => r.status == 201 });
}

