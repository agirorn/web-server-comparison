import * as dotenv from 'dotenv';
import express from 'express';
import { Request, Response } from 'express';
import bodyParser from 'body-parser';
import { SQL } from 'sql-template-strings';
import pg from 'pg';

dotenv.config();
const {
  DB_DATABASE,
  DB_HOST,
  DB_USER,
  DB_PASSWORD,
  DB_MAX,
  DB_IDLE_TIMEOUT_MILLIS,
  DB_CONNECTION_TIMEOUT_MILLIS,
} = process.env;

const { Pool } = pg;

const app = express();
// parse various different custom JSON types as JSON
app.use(bodyParser.json({ type: 'application/json' }));

const parseInt = (v: string | undefined, defaultValue: number): number =>
  Number.parseInt(DB_IDLE_TIMEOUT_MILLIS || `${defaultValue}`, 10);

const port = 3000;
// const { Pool } = pg;
const pool = new Pool({
  database: DB_DATABASE,
  host: DB_HOST,
  user: DB_USER,
  password: DB_PASSWORD,
  max: parseInt(DB_MAX, 50),
  idleTimeoutMillis: parseInt(DB_IDLE_TIMEOUT_MILLIS, 30000),
  connectionTimeoutMillis: parseInt(DB_CONNECTION_TIMEOUT_MILLIS, 2000),
});
pool
  .query('TRUNCATE events RESTART IDENTITY;')
  .then(() => {
    // eslint-disable-next-line no-console
    console.log('The events table has been reset');
  })
  .catch((error) => {
    // eslint-disable-next-line no-console
    console.log(error);
    // eslint-disable-next-line no-process-exit
    process.exit(1);
  });

// eslint-disable-next-line no-console
const printDot = (): void => console.log('.');
process.nextTick(printDot);
app.get('/', (req: Request, res: Response) => {
  res.send('This is a test server');
});

app.get('/hello-world', (req: Request, res: Response) => {
  res.send('Hello World!');
});

app.post('/save', (req: Request, res: Response) => {
  pool
    .query(
      SQL`
     INSERT INTO events (time)
     VALUES (
       ${new Date().toISOString()}
     )
   `
    )
    .then(() => {
      res.json({ ok: true });
    })
    .catch((err) => {
      res.json(err.message);
    });
});

app.listen(port, () => {
  // eslint-disable-next-line no-console
  console.log(`app pid: (${process.pid}) listening on port ${port}`);
});
