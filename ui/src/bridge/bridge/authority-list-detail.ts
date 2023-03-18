import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { EntryHash, Record, ActionHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { Task } from '@lit-labs/task';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import './edit-authority-list';

import { clientContext } from '../../contexts';
import { AuthorityList } from './types';

@customElement('authority-list-detail')
export class AuthorityListDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  authorityListHash!: ActionHash;

  _fetchRecord = new Task(this, ([authorityListHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'bridge',
      zome_name: 'bridge',
      fn_name: 'get_authority_list',
      payload: authorityListHash,
  }) as Promise<Record | undefined>, () => [this.authorityListHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.authorityListHash === undefined) {
      throw new Error(`The authorityListHash property is required for the authority-list-detail element`);
    }
  }


  renderDetail(record: Record) {
    const authorityList = decode((record.entry as any).Present.entry) as AuthorityList;

    return html`
      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
        </div>

      </div>
    `;
  }
  
  renderAuthorityList(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested authority list was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-authority-list
    	  .currentRecord=${maybeRecord}
    	  @authority-list-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-authority-list>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderAuthorityList(maybeRecord),
      error: (e: any) => html`<span>Error fetching the authority list: ${e.data.data}</span>`
    });
  }
}
