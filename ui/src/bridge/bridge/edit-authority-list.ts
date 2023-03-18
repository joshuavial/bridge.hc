import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { ActionHash, EntryHash, AgentPubKey, Record, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { AuthorityList } from './types';

@customElement('edit-authority-list')
export class EditAuthorityList extends LitElement {

  @consume({ context: clientContext })
  client!: AppAgentClient;
  
  
  @property()
  currentRecord!: Record;
 
  get currentAuthorityList() {
    return decode((this.currentRecord.entry as any).Present.entry) as AuthorityList;
  }
 

  isAuthorityListValid() {
    return true;
  }
  
  connectedCallback() {
    super.connectedCallback();
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord property is required for the edit-authority-list element`);
    }
    
  }

  async updateAuthorityList() {
    const authorityList: AuthorityList = { 
      percentage_for_consensus: this.currentAuthorityList.percentage_for_consensus,
      authorities: this.currentAuthorityList.authorities,
    };

    try {
      const updateRecord: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'bridge',
        zome_name: 'bridge',
        fn_name: 'update_authority_list',
        payload: {
          previous_authority_list_hash: this.currentRecord.signed_action.hashed.hash,
          updated_authority_list: authorityList
        },
      });
  
      this.dispatchEvent(new CustomEvent('authority-list-updated', {
        composed: true,
        bubbles: true,
        detail: {
          previousAuthorityListHash: this.currentRecord.signed_action.hashed.hash,
          updatedAuthorityListHash: updateRecord.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('update-error') as Snackbar;
      errorSnackbar.labelText = `Error updating the authority list: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="update-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Edit Authority List</span>


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
            .disabled=${!this.isAuthorityListValid()}
            @click=${() => this.updateAuthorityList()}
            style="flex: 1;"
          ></mwc-button>
        </div>
      </div>`;
  }
}
