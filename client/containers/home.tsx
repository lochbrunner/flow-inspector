import { RootState } from '../reducers/model';
import * as React from 'react';
import { connect } from 'react-redux';

import * as style from './home.scss';

interface State {

}

interface Actions {

}

type Props = State & Actions;

const render = (props: Props) => {
    return (
        <div>
            <h1 className={style.title}>Home Container</h1>
        </div>
    );
};

const mapStateToProps = (state: RootState): State => ({

});

const mapDispatchToProps = (dispatch: any): Actions => ({

});

export default connect(
    mapStateToProps,
    mapDispatchToProps,
)(render);

// export default ScenarioContainer;