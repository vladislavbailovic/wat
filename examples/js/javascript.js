import Api from './api';

module.exports = context => {

	const request = context.request;

	const Cache = require( './cache' )( context );
	const Member = require( './member' )( context );

	return class Hub {

			 // TODO: ensure immutability if privates.
			 // 
			 // Idk a comment of some sort goes here.
			 // Whatever, man.
			 // ---
			 // Another paragraph.
			static RQ_LIST_SITES = 'getSites';
			static RQ_GET_SITE = 'getSiteInfo';
			static RQ_PREPARE_PUSH = 'preparePush';
			static RQ_CLEANUP_PUSH = 'cleanupPush';

			constructor( site ) {
					this.site = site;
					this.member = new Member( site );
			}

			getSites() {
					// @TODO remove stub!
					return Promise.resolve(
							[ 'body-exposure' ] // TODO!! don't hardcode
					);
					// TODO: use this instead
					return new Promise( ( resolve, reject ) => {
							this.getData( Hub.RQ_LIST_SITES )
									.then( data => resolve( data || [] ) )
									.catch( reject )
							;
					} );
			}
	}
}
