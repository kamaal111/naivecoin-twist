import {get, post} from '../utils/request';

import type {BlockType, Result} from '../types';

const BASE_URL = 'http://127.0.0.1:8080';

class BlocksClient {
  public path = '/blocks';

  constructor() {}

  public async getAll(): Promise<Result<BlockType[]>> {
    return get({url: this.makeURL()});
  }

  public async getLatest(): Promise<Result<BlockType[]>> {
    return get({url: this.makeURL('?latest=1')});
  }

  public async addToChain(payload: BlockType): Promise<Result<undefined>> {
    return post({url: this.makeURL(), payload});
  }

  public async replaceChain(payload: BlockType[]): Promise<Result<undefined>> {
    return post({url: this.makeURL('/replace-chain'), payload});
  }

  public async mineBlock(payload: string): Promise<Result<BlockType>> {
    return post({url: this.makeURL('/mine'), payload: {data: payload}});
  }

  private makeURL(extension = '') {
    return `${BASE_URL}${this.path}${extension}`;
  }
}

class CoreClient {
  public blocks = new BlocksClient();

  constructor() {}
}

export default CoreClient;
