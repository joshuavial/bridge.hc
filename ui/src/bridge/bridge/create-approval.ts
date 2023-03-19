import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

import { clientContext } from '../../contexts';
import { Approval } from './types';

@customElement('create-approval')
export class CreateApproval extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  authorityList!: EntryHash;
  @property()
  approvedBy!: Array<AgentPubKey>;


  @state()
  _timestamp: number = Date.now();

  
  firstUpdated() {
    if (this.authorityList === undefined) {
      throw new Error(`The authorityList input is required for the create-approval element`);
    }
    if (this.approvedBy === undefined) {
      throw new Error(`The approvedBy input is required for the create-approval element`);
    }
  }

  isApprovalValid() {
    return true && true;
  }

  async createApproval() {
    const approval: Approval = { 
        authority_list: this.authorityList,
        timestamp: this._timestamp,
        approved_by: this.approvedBy,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'bridge',
        zome_name: 'bridge',
        fn_name: 'create_approval',
        payload: approval,
      });

      this.dispatchEvent(new CustomEvent('approval-created', {
        composed: true,
        bubbles: true,
        detail: {
          approvalHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the approval: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Approval</span>

          <div style="margin-bottom: 16px">
            <vaadin-date-time-picker label="Timestamp" .value=${new Date(this._timestamp / 1000).toISOString()} @change=${(e: CustomEvent) => { this._timestamp = new Date((e.target as any).value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
          </div>
            

        <mwc-button 
          raised
          label="Create Approval"
          .disabled=${!this.isApprovalValid()}
          @click=${() => this.createApproval()}
        ></mwc-button>
    </div>`;
  }
}
