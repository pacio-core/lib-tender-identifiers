package com.pacio.rusty_app

import android.os.Bundle
import android.support.design.widget.Snackbar
import android.support.v7.app.AppCompatActivity;
import android.view.Menu
import android.view.MenuItem
import android.widget.TextView

import kotlinx.android.synthetic.main.activity_main.*
import com.pacio.rustylibrary.helloDirect
import com.pacio.rustylibrary.loadRustyLib

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        setSupportActionBar(toolbar)

        loadRustyLib()

//        findViewById<TextView>(R.id.txt).let {
//            it?.text =  hello("Rob")
//        }
        findViewById<TextView>(R.id.txt).let {
            it?.text =  helloDirect("Hello Luka")
        }
//        var greeting2 = helloDirect("Rob Direct")

        fab.setOnClickListener { view ->
            Snackbar.make(view, "GREETING2", Snackbar.LENGTH_LONG)
                .setAction("Action", null).show()
        }
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.menu_main, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        return when (item.itemId) {
            R.id.action_settings -> true
            else -> super.onOptionsItemSelected(item)
        }
    }
}
