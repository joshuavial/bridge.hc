import { LitElement, html, css } from 'lit';
import { Task } from '@lit-labs/task';
import { state, customElement, property } from 'lit/decorators.js';
import { consume } from '@lit-labs/context';

import { decode } from '@msgpack/msgpack';
import { EntryHash, Record, ActionHash, AppAgentClient, DnaHash } from '@holochain/client';

import { AuthorityList } from '../types';
import { clientContext } from '../contexts';

@customElement('footer-component')
class FooterComponent extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  authorityListHash!: ActionHash;

  _fetchRecord = new Task(this, () => this.client.callZome({
    cap_secret: null,
    role_name: 'bridge',
    zome_name: 'bridge',
    fn_name: 'get_authority_list',
    payload: null,
}) as Promise<Record | undefined>, () => [this.authorityListHash]);
  
  renderAuthorityList(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested authority list was not found.</span>`;

    const authorityList = decode((maybeRecord.entry as any).Present.entry) as AuthorityList;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1">List</span>
        </div>
      </div>
    `;
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <sl-spinner></sl-spinner>
      </div>`,
      complete: (maybeRecord) => this.renderAuthorityList(maybeRecord),
      error: (e: any) => html`<span>Error fetching the authority list: ${e.data.data}</span>`
    });
  }

  static styles = css`
    :host {
      background-color: #a3f3f3;
      flex-grow: 1;
      display: flex;
    }
`;
}
