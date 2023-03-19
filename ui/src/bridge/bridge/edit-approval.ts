import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import { clientContext } from '../../contexts';
import { Approval } from './types';

@customElement('edit-approval')
export class EditApproval extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  @property({
      hasChanged: (newVal: ActionHash, oldVal: ActionHash) => newVal?.toString() !== oldVal?.toString()
  })
  originalApprovalHash!: ActionHash;

  
  @property()
  currentRecord!: Record;
 
  get currentApproval() {
    return decode((this.currentRecord.entry as any).Present.entry) as Approval;
  }
 
  @state()
  _timestamp!: number;


  isApprovalValid() {
    return true && true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-approval element`);
    }

    if (this.originalApprovalHash === undefined) {
      throw new Error(`The originalApprovalHash property is required for the edit-approval element`);
    }
    
    this._timestamp = this.currentApproval.timestamp;
  }

  async updateApproval() {
    const approval: Approval = { 
      timestamp: this._timestamp!,
      authority_list: this.currentApproval.authority_list,
      approved_by: this.currentApproval.approved_by,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'bridge',
        zome_name: 'bridge',
        fn_name: 'update_approval',
        payload: {
          original_approval_hash: this.originalApprovalHash,
          previous_approval_hash: this.currentRecord.signed_action.hashed.hash,
          updated_approval: approval
        },
      });
  
      this.dispatchEvent(new CustomEvent('approval-updated', {
        composed: true,
        bubbles: true,
        detail: {
          originalApprovalHash: this.originalApprovalHash,
          previousApprovalHash: this.currentRecord.signed_action.hashed.hash,
          updatedApprovalHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the approval: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Approval</span>
          <div style="margin-bottom: 16px">
          <vaadin-date-time-picker label="Timestamp" .value=${new Date(this._timestamp / 1000).toISOString()} @change=${(e: CustomEvent) => { this._timestamp = new Date((e.target as any).value).valueOf() * 1000;} } required></vaadin-date-time-picker>    
          </div>



        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            @click=${() => this.dispatchEvent(new CustomEvent('edit-canceled', {
              bubbles: true,
              composed: true
            }))}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>
          <mwc-button 
            raised
            label="Save"
            .disabled=${!this.isApprovalValid()}
            @click=${() => this.updateApproval()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
