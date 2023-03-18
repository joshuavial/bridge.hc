import { LitElement, html } from 'lit';
import { state, customElement, property } from 'lit/decorators.js';
import { InstalledCell, ActionHash, Record, AgentPubKey, EntryHash, AppAgentClient, DnaHash } from '@holochain/client';
import { consume } from '@lit-labs/context';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import { clientContext } from '../../contexts';
import { AuthorityList } from './types';

@customElement('create-authority-list')
export class CreateAuthorityList extends LitElement {
  @consume({ context: clientContext })
  client!: AppAgentClient;

  @property()
  percentageForConsensus!: number;
  @property()
  authorities!: Array<string>;


  
  firstUpdated() {
    if (this.percentageForConsensus === undefined) {
      throw new Error(`The percentageForConsensus input is required for the create-authority-list element`);
    }
    if (this.authorities === undefined) {
      throw new Error(`The authorities input is required for the create-authority-list element`);
    }
  }

  isAuthorityListValid() {
    return true;
  }

  async createAuthorityList() {
    const authorityList: AuthorityList = { 
        percentage_for_consensus: this.percentageForConsensus,
        authorities: this.authorities,
    };

    try {
      const record: Record = await this.client.callZome({
        cap_secret: null,
        role_name: 'bridge',
        zome_name: 'bridge',
        fn_name: 'create_authority_list',
        payload: authorityList,
      });

      this.dispatchEvent(new CustomEvent('authority-list-created', {
        composed: true,
        bubbles: true,
        detail: {
          authorityListHash: record.signed_action.hashed.hash
        }
      }));
    } catch (e: any) {
      const errorSnackbar = this.shadowRoot?.getElementById('create-error') as Snackbar;
      errorSnackbar.labelText = `Error creating the authority list: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  render() {
    return html`
      <mwc-snackbar id="create-error" leading>
      </mwc-snackbar>

      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create Authority List</span>


        <mwc-button 
          raised
          label="Create Authority List"
          .disabled=${!this.isAuthorityListValid()}
          @click=${() => this.createAuthorityList()}
        ></mwc-button>
    </div>`;
  }
}
