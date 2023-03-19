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

import './edit-approval';

import { clientContext } from '../../contexts';
import { Approval } from './types';

@customElement('approval-detail')
export class ApprovalDetail extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property({
    hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  approvalHash!: ActionHash;

  _fetchRecord = new Task(this, ([approvalHash]) => this.client.callZome({
      cap_secret: null,
      role_name: 'bridge',
      zome_name: 'bridge',
      fn_name: 'get_approval',
      payload: approvalHash,
  }) as Promise<Record | undefined>, () => [this.approvalHash]);

  @state()
  _editing = false;
  
  firstUpdated() {
    if (this.approvalHash === undefined) {
      throw new Error(`The approvalHash property is required for the approval-detail element`);
    }
  }

  async deleteApproval() {
    try {
      await this.client.callZome({
        cap_secret: null,
        role_name: 'bridge',
        zome_name: 'bridge',
        fn_name: 'delete_approval',
        payload: this.approvalHash,
      });
      this.dispatchEvent(new CustomEvent('approval-deleted', {
        bubbles: true,
        composed: true,
        detail: {
          approvalHash: this.approvalHash
        }
      }));
      this._fetchRecord.run();
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('delete-error') as Snackbar;
      errorSnackbar.labelText = `Error deleting the approval: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  renderDetail(record: Record) {
    const approval = decode((record.entry as any).Present.entry) as Approval;

    return html`
      <mwc-snackbar id="delete-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
      	<div style="display: flex; flex-direction: row">
      	  <span style="flex: 1"></span>
      	
          <mwc-icon-button style="margin-left: 8px" icon="edit" @click=${() => { this._editing = true; } }></mwc-icon-button>
          <mwc-icon-button style="margin-left: 8px" icon="delete" @click=${() => this.deleteApproval()}></mwc-icon-button>
        </div>

        <div style="display: flex; flex-direction: row; margin-bottom: 16px">
	  <span style="margin-right: 4px"><strong>Timestamp: </strong></span>
 	  <span style="white-space: pre-line">${new Date(approval.timestamp / 1000).toLocaleString() }</span>
        </div>

      </div>
    `;
  }
  
  renderApproval(maybeRecord: Record | undefined) {
    if (!maybeRecord) return html`<span>The requested approval was not found.</span>`;
    
    if (this._editing) {
    	return html`<edit-approval
    	  .originalApprovalHash=${this.approvalHash}
    	  .currentRecord=${maybeRecord}
    	  @approval-updated=${async () => {
    	    this._editing = false;
    	    await this._fetchRecord.run();
    	  } }
    	  @edit-canceled=${() => { this._editing = false; } }
    	  style="display: flex; flex: 1;"
    	></edit-approval>`;
    }

    return this.renderDetail(maybeRecord);
  }

  render() {
    return this._fetchRecord.render({
      pending: () => html`<div style="display: flex; flex: 1; align-items: center; justify-content: center">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>`,
      complete: (maybeRecord) => this.renderApproval(maybeRecord),
      error: (e: any) => html`<span>Error fetching the approval: ${e.data.data}</span>`
    });
  }
}
