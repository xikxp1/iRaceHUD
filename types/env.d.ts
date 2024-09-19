export {};

declare global {
  namespace NodeJS {
    interface ProcessEnv {
      IRACING_LOGIN: string;
      IRACING_PWD: string;
    }
  }
}
