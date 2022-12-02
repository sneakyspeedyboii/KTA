import Link from "next/link";
import Home_icon from "../components/icons/home_icon.jsx";
import Flag_icon from "../components/icons/flag_icon.jsx";
import Graph_icon from "../components/icons/graph_icon.jsx";
import Swim_icon from "../components/icons/swim_icon.jsx";

function layout({ children }) {
    return (
        <div className="home">
            <ul className="nav">
                <li>
                    <Link className="navcontent" href="/">
                        <a>
                            <Home_icon size="40" /> 
                        </a>
                    </Link>
                </li>
                <li>
                    <Link className="navcontent" href="/graph">
                        <a>
                            <Graph_icon size="40" />
                        </a>
                    </Link>
                </li>
                <li>
                    <Link className="navcontent" href="/sessionretr">
                        <a>
                            <Flag_icon size="40" />
                        </a>
                    </Link>
                </li>
                <li>
                    <Link className="navcontent" href="/rustlang" >
                        <a>
                            <Swim_icon size="40" />
                        </a>
                    </Link>
                </li>
            </ul>

            <div className="layout_content">
                {children}
            </div>
        </div>
    );
}

export default layout;